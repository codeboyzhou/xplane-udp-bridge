import 'package:flutter/material.dart';
import 'package:flutters/viewmodel/udp.dart';

import '../helper.dart';

class DataRefViewModel extends ChangeNotifier {
  static const String parkingBrakeRatio = 'sim/cockpit2/controls/parking_brake_ratio';
  static const String throttleRatio = 'sim/cockpit2/engine/actuators/throttle_ratio';
  static const String engineMaster = 'sim/cockpit2/engine/actuators/eng_master';
  static const String batteryOn = 'sim/cockpit2/electrical/battery_on';

  static const Map<String, String> datarefs = {
    parkingBrakeRatio: 'float',
    throttleRatio: 'float',
    engineMaster: '[int]',
    batteryOn: '[int]',
  };

  static const Map<String, String> datarefValues = {
    parkingBrakeRatio: '',
    throttleRatio: '',
    engineMaster: '',
    batteryOn: '',
  };

  final UdpViewModel _udpViewModel;

  DataRefViewModel(this._udpViewModel);

  void readFromUDPServer(String host, int port) {
    final String serverAddress = '$host:$port';
    datarefs.forEach((dataref, datatype) {
      final data = 'dataref|read|$datatype|$dataref';
      logger.i('Sending dataref read request to $serverAddress: $data');
      _udpViewModel.send(host, port, data);
    });
  }

  String parse(String message) {
    final parts = message.split('|');
    if (parts.length == 4 && parts[0] == 'dataref' && parts[1] == 'read') {
      final dataref = parts[3];
      final value = parts[2];
      if (datarefs.containsKey(dataref)) {
        datarefValues[dataref] = value;
        notifyListeners();
      }
    }
    return message;
  }
}
