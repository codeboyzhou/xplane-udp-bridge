import 'package:flutter/material.dart';

import '../data/udp.dart';

class UdpViewModel extends ChangeNotifier {
  final UdpClient _client = UdpClient();

  String _host = '127.0.0.1';

  String get host => _host;

  int _port = 49000;

  int get port => _port;

  bool _isRunning = false;

  bool get isRunning => _isRunning;

  final List<String> _messages = [];

  List<String> get messages => List.unmodifiable(_messages);

  Future<void> start() async {
    if (_isRunning) {
      return;
    }

    await _client.bind();

    _client.onMessage = (message) {
      _messages.add(message);
      notifyListeners();
    };

    _isRunning = true;
    notifyListeners();
  }

  void send(String host, int port, String message) {
    _host = host;
    _port = port;
    _client.send(message, _host, _port);
  }
}
