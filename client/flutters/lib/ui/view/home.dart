import 'package:flutter/material.dart';
import 'package:flutters/helper.dart';
import 'package:provider/provider.dart';

import '../../viewmodel/dataref.dart';
import '../../viewmodel/udp.dart';

class HomeScreenView extends StatefulWidget {
  const HomeScreenView({super.key, required this.title});

  // This widget is the home screen view of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<HomeScreenView> createState() => _HomeScreenViewState();
}

class _HomeScreenViewState extends State<HomeScreenView> {
  final TextEditingController _udpServerIp = TextEditingController();
  final TextEditingController _udpServerPort = TextEditingController();

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      context.read<UdpViewModel>().start();
    });
  }

  @override
  Widget build(BuildContext context) {
    final udpViewModel = context.watch<UdpViewModel>();
    final datarefViewModel = context.watch<DataRefViewModel>();

    // This method is rerun every time setState is called.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Scaffold(
      appBar: AppBar(
        // backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        // Here we take the value from the HomeScreen object that was created by
        // the App.build method, and use it to set our appbar title.
        title: Text(widget.title),
      ),
      body: Padding(
        padding: const EdgeInsets.only(left: 16.0, right: 16.0, top: 16.0, bottom: 96.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              child: ListView.builder(
                itemCount: udpViewModel.messages.length,
                itemBuilder: (context, index) =>
                    ListTile(title: Text(udpViewModel.messages[index])),
              ),
            ),

            Align(
              alignment: Alignment.center,
              child: Column(
                children: [
                  TextFormField(
                    controller: _udpServerIp,
                    decoration: const InputDecoration(
                      labelText: 'Input UDP server IP here',
                      hintText: 'e.g. 192.168.1.100',
                      border: OutlineInputBorder(),
                    ),
                    keyboardType: TextInputType.numberWithOptions(decimal: true),
                  ),
                  const SizedBox(height: 16),
                  TextFormField(
                    controller: _udpServerPort,
                    decoration: const InputDecoration(
                      labelText: 'Input UDP server port here',
                      hintText: 'e.g. 49000',
                      border: OutlineInputBorder(),
                    ),
                    keyboardType: TextInputType.number,
                  ),
                  const SizedBox(height: 16),
                  ElevatedButton(
                    onPressed: () {
                      final ip = _udpServerIp.text.trim();
                      if (ip.isEmpty) {
                        showSnackBar(context, 'Please input a valid IPv4 address');
                        return;
                      }

                      final port = int.tryParse(_udpServerPort.text.trim());
                      if (port == null) {
                        showSnackBar(context, 'Please input a valid port number');
                        return;
                      }

                      showSnackBar(context, 'Sending dataref read request to server $ip:$port');
                      datarefViewModel.readFromUDPServer(ip, port);
                    },
                    style: ElevatedButton.styleFrom(
                      textStyle: const TextStyle(fontSize: 16.0, fontWeight: FontWeight.bold),
                    ),
                    child: const Text('Click to Read Datarefs from UDP Server'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
