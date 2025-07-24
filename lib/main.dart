import 'dart:developer';

import 'package:rinf/rinf.dart';
import 'src/bindings/bindings.dart';
import 'package:flutter/material.dart';

Future<void> main() async {
  await initializeRust(assignRustSignal);
  runApp(const App());
}

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Fernetable',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
      ),
      home: const HomePage(),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final TextEditingController _configController = TextEditingController();
  String? _key;
  String? _encryptedConfig;
  String? _decryptedConfig;

  @override
  void initState() {
    super.initState();
    CommandResponse.rustSignalStream.listen(
      (event) {
        setState(() {
          if (event.message.encryptedConfig != null) {
            _encryptedConfig = event.message.encryptedConfig;
          }
          if (event.message.decryptedConfig != null) {
            _decryptedConfig = event.message.decryptedConfig;
          }
          if (event.message.key != null) {
            _key = event.message.key;
          }
        });
      },
      onError: (error) {
        log('Error: $error');
      },
      onDone: () {
        log('Done');
      },
    );
  }

  @override
  void dispose() {
    _configController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          TextField(
            controller: _configController,
            decoration: const InputDecoration(labelText: 'Config'),
          ),
          (_key == null)
              ? ElevatedButton(
                  onPressed: () {
                    CommandRequest(
                      command: Command.generateKey,
                    ).sendSignalToRust();
                  },
                  child: const Text('Generate Key'),
                )
              : Text('Key: $_key'),
          ElevatedButton(
            onPressed: () {
              CommandRequest(
                command: Command.encryptConfig,
                config: _configController.text,
                key: _key ?? '',
              ).sendSignalToRust();
            },
            child: const Text('Encrypt'),
          ),
          Text(
            'Encrypted config: ${_encryptedConfig ?? 'Waiting for encryption...'}',
          ),
          ElevatedButton(
            onPressed: () {
              CommandRequest(
                command: Command.decryptConfig,
                config: _encryptedConfig ?? '',
                key: _key ?? '',
              ).sendSignalToRust();
            },
            child: const Text('Decrypt'),
          ),
          Text(
            'Decrypted config: ${_decryptedConfig ?? 'Waiting for decryption...'}',
          ),
        ],
      ),
    );
  }
}
