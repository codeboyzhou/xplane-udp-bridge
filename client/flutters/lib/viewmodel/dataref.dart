import 'package:flutter/material.dart';
import 'package:flutters/viewmodel/udp.dart';
import 'package:uuid/uuid.dart';

import '../helper.dart';

class DataRef {
  final String name;

  final String datatype;

  final String description;

  DataRef(this.name, this.datatype, this.description);
}

class DataRefViewModel extends ChangeNotifier {
  final List<DataRef> datarefs = [
    DataRef('sim/cockpit2/controls/parking_brake_ratio', 'float', 'Parking Brake Ratio'),
    DataRef('sim/cockpit2/engine/actuators/throttle_ratio', 'float', 'Throttle Ratio'),
    DataRef('sim/cockpit2/engine/actuators/eng_master', '[int]', 'Engine Master'),
    DataRef('sim/cockpit2/electrical/battery_on', '[int]', 'Battery On'),
  ];

  // key: requestId, value: dataref name
  final Map<String, String> requestIds = {};

  final UdpViewModel _udpViewModel;

  DataRefViewModel(this._udpViewModel);

  String _generateRequestId() {
    return Uuid().v4().replaceAll('-', '');
  }

  void readFromUDPServer(String host, int port) {
    final String serverAddress = '$host:$port';
    for (final dataref in datarefs) {
      final requestId = _generateRequestId();
      final data = '$requestId|dataref|read|${dataref.datatype}|${dataref.name}';
      logger.i('Sending dataref read request to $serverAddress: $data');
      requestIds[requestId] = dataref.name;
      _udpViewModel.send(host, port, data);
    }
  }

  String parse(String message) {
    final parts = message.split('|');
    final requestId = parts[0];
    final value = parts[3];
    final datarefName = requestIds[requestId];
    final dataref = datarefs.firstWhere((element) => element.name == datarefName);
    return '${dataref.description}: $value';
  }
}
