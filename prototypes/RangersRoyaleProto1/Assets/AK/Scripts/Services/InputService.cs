using AK.Scripts.Components;
using AK.Scripts.Entities;
using UnityEngine;
using UnityEngine.InputSystem;
using Zenject;

namespace AK.Scripts.Services
{
    public class InputService : ITickable
    {
        private readonly PlayerSquad _playerSquad;

        public InputService(PlayerSquad playerSquad)
        {
            _playerSquad = playerSquad;
        }

        void ITickable.Tick()
        {
            var keyboard = Keyboard.current;
            if (keyboard.f2Key.wasPressedThisFrame) 
                _playerSquad.SelectAll();
            if (keyboard.digit1Key.wasPressedThisFrame) 
                _playerSquad.Select(0);
            if (keyboard.digit2Key.wasPressedThisFrame) 
                _playerSquad.Select(1);
            if (keyboard.digit3Key.wasPressedThisFrame) 
                _playerSquad.Select(2);
            if (keyboard.digit4Key.wasPressedThisFrame) 
                _playerSquad.Select(3);
            
        }

        public void OnGroundClick(Vector2 position) => _playerSquad.CommandMove(position);
    }
}