using System;
using UnityEngine.InputSystem;
using Zenject;

namespace AK.Scripts.Services
{
    public class PauseService : ITickable
    {
        public int Ticks { get; set; }
        public bool Paused { get; private set; }

        public event Action PausedChanged;

        public void Tick()
        {
            var keyboard = Keyboard.current;
            if (keyboard.spaceKey.wasPressedThisFrame)
            {
                Paused = !Paused;
                PausedChanged?.Invoke();
            }

            if (!Paused) 
                Ticks++;
        }
    }
}