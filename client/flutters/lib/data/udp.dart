import 'dart:io';

import '../helper.dart';

class UdpClient {
  RawDatagramSocket? _socket;

  void Function(String received)? onMessage;

  Future<void> bind() async {
    _socket = await RawDatagramSocket.bind(InternetAddress.anyIPv4, 0);
    _socket!.listen((event) {
      if (event == RawSocketEvent.read) {
        final datagram = _socket!.receive();
        if (datagram != null) {
          final message = String.fromCharCodes(datagram.data);
          logger.d('Parsed datagram data as message: $message');
          onMessage?.call(message);
        }
      }
    });
  }

  void send(String message, String host, int port) {
    _socket?.send(message.codeUnits, InternetAddress(host), port);
  }
}
