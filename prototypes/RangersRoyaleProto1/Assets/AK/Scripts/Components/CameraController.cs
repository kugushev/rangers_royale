using System;
using UnityEngine;
using UnityEngine.InputSystem;

namespace AK.Scripts.Components
{
    public class CameraController: MonoBehaviour
    {
        private float defaultOrthographicSize;
        private Camera _camera;
        private void Awake()
        {
            _camera = GetComponent<Camera>();
            defaultOrthographicSize = _camera.orthographicSize;
        }

        private float _lastScroll = 0f;
        
        private void Update()
        {
            // var keyboard = Keyboard.current;
            // var moveVec = Vector3.zero;
            // if (keyboard.wKey.wasPressedThisFrame)
            // {
            //     moveVec.y += 0.1f;
            // }

            var mouse = Mouse.current.scroll.ReadValue();
            if (_lastScroll != 0f)
            {
                var delta = mouse.y - _lastScroll;
                var newValue = _camera.orthographicSize + delta * 0.01f;
                _camera.orthographicSize = Mathf.Max(newValue, defaultOrthographicSize);
            }

            _lastScroll = mouse.y;
        }
    }
}