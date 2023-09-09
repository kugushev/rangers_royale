using System.Collections.Generic;
using AK.Scripts.Entities;
using AK.Scripts.Entities.Units;
using TMPro;
using UnityEngine;
using UnityEngine.UI;
using Zenject;

namespace AK.Scripts.Services
{
    public class TimeMachine : MonoBehaviour
    {
        public Slider slider;
        public TextMeshProUGUI ticks;
        [Inject] private PauseService _pauseService;
        [Inject] private PlayerSquad _squad;

        private void Awake()
        {
            _pauseService.PausedChanged += PauseChanged;

            slider.onValueChanged.AddListener(OnSlide);
        }

        private void Update()
        {
            ticks.text = _pauseService.Ticks.ToString();
        }

        private void PauseChanged()
        {
            if (_pauseService.Paused)
            {
                slider.gameObject.SetActive(true);
                slider.minValue = 0f;
                slider.maxValue = _pauseService.Ticks;
                slider.value = _pauseService.Ticks;
            }
            else
            {
                slider.gameObject.SetActive(false);

                foreach (var hero in _squad.AllHeroes)
                    hero.TrimStory(_pauseService.Ticks, out _);

                var mobsToDelete = new List<Mob>();
                foreach (var mob in Mob.AllMobs)
                {
                    mob.TrimStory(_pauseService.Ticks, out var notBorn);
                    if (notBorn) 
                        mobsToDelete.Add(mob);
                }

                foreach (var mob in mobsToDelete)
                {
                    Destroy(mob.gameObject);
                    Mob.AllMobs.Remove(mob);
                }
            }
        }

        private void OnSlide(float value)
        {
            _pauseService.Ticks = (int) value;

            foreach (var hero in _squad.AllHeroes)
                hero.RestoreState(_pauseService.Ticks, out _);

            foreach (var mob in Mob.AllMobs)
                mob.RestoreState(_pauseService.Ticks, out _);
        }
    }
}