package com.example.fpvjoystick

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.zerokol.views.JoystickView

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val joystickLeft = findViewById<JoystickView>(R.id.joystickLeft)
        val joystickRight = findViewById<JoystickView>(R.id.joystickRight)

        joystickLeft.setOnMoveListener { angle, strength ->
            // angle = направление, strength = сила движения
            // TODO: Отправить данные левому движению дрона
            println("Левый джойстик: angle=$angle, strength=$strength")
        }

        joystickRight.setOnMoveListener { angle, strength ->
            // TODO: Отправить данные правому движению дрона
            println("Правый джойстик: angle=$angle, strength=$strength")
        }
    }
}
