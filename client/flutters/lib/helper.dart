import 'package:flutter/material.dart';
import 'package:logger/logger.dart';

final Logger logger = Logger(printer: SimplePrinter(colors: false));

void showSnackBar(BuildContext context, String message) {
  ScaffoldMessenger.of(context).hideCurrentSnackBar();
  ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
}
