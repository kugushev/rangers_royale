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
            if (keyboard.oem1Key.wasPressedThisFrame) 
                _playerSquad.Select(1);
            if (keyboard.oem2Key.wasPressedThisFrame) 
                _playerSquad.Select(2);
            if (keyboard.oem3Key.wasPressedThisFrame) 
                _playerSquad.Select(3);
            if (keyboard.oem4Key.wasPressedThisFrame) 
                _playerSquad.Select(4);
            
        }

        public void OnGroundClick(Vector2 position) => _playerSquad.CommandMove(position);
    }
}