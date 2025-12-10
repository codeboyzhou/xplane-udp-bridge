import 'package:flutter/material.dart';
import 'package:flutters/ui/view/home.dart';
import 'package:flutters/viewmodel/dataref.dart';
import 'package:flutters/viewmodel/udp.dart';
import 'package:provider/provider.dart';

void main() {
  runApp(
    MultiProvider(
      providers: [
        ChangeNotifierProvider(create: (context) => UdpViewModel()),
        ChangeNotifierProvider(create: (context) => DataRefViewModel(context.read<UdpViewModel>())),
      ],
      child: const App(),
    ),
  );
}

class App extends StatelessWidget {
  const App({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(colorScheme: ColorScheme.fromSeed(seedColor: Colors.lightBlue)),
      home: const HomeScreenView(title: 'X-Plane UDP Bridge Plugin Client'),
    );
  }
}
