using AK.Scripts.Services;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.Tilemaps;
using Zenject;

namespace AK.Scripts.Components
{
    [RequireComponent(typeof(TilemapCollider2D))]
    public class GroundClickHandler : MonoBehaviour, IPointerClickHandler
    {
        [Inject] private readonly InputService _inputService;

        private TilemapCollider2D _tilemapCollider2D;
        private Camera _camera = default!;

        private void Awake()
        {
            _camera = Camera.main!;
            _tilemapCollider2D = GetComponent<TilemapCollider2D>();
        }

        public void OnPointerClick(PointerEventData eventData)
        {
            if (eventData.button == PointerEventData.InputButton.Right)
            {
                var position = _camera.ScreenToWorldPoint(eventData.position);
                _inputService.OnGroundClick(position);
            }
        }
    }
}