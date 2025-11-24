package com.example.fpvdrone

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import io.github.controlwear.virtual.joystick.android.JoystickView

class MainActivity : AppCompatActivity() {

    private lateinit var leftJoy: JoystickView
    private lateinit var rightJoy: JoystickView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        leftJoy = findViewById(R.id.joystickLeft)
        rightJoy = findViewById(R.id.joystickRight)

        leftJoy.setOnMoveListener { angle, strength ->
            val throttle = strength
            val yaw = angle
            sendToDrone("THROTTLE", throttle)
            sendToDrone("YAW", yaw)
        }

        rightJoy.setOnMoveListener { angle, strength ->
            val pitch = strength
            val roll = angle
            sendToDrone("PITCH", pitch)
            sendToDrone("ROLL", roll)
        }
    }

    private fun sendToDrone(type: String, value: Int) {
        println("$type: $value")
    }
}
