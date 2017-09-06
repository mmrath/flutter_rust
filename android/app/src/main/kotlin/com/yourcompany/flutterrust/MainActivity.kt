package com.yourcompany.flutterrust

import android.os.Bundle

import io.flutter.app.FlutterActivity
import io.flutter.plugins.GeneratedPluginRegistrant
import io.flutter.plugin.common.MethodChannel.Result
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel


class MainActivity() : FlutterActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        GeneratedPluginRegistrant.registerWith(this)

        MethodChannel(getFlutterView(), HELLO_CHANNEL).setMethodCallHandler(
                object : MethodCallHandler {
                    var counter = 0;
                    @Override
                    override fun onMethodCall(call: MethodCall, result: Result) {
                        if (call.method.equals("sayHello")) {
                            val arg  = call.arguments<String>()
                            result.success(hello(arg)+counter++)
                        } else {
                            result.notImplemented()
                        }
                    }
                }
        )
    }

    companion object {
        val HELLO_CHANNEL = "channel/hello"
        @JvmStatic external fun hello(name: String) : String
        init{
            System.loadLibrary("flutter_rust_demo")
        }
    }

}
