using System;
using System.Collections.Generic;
using AK.Scripts.Entities.Units;
using UnityEngine;
using Zenject;

namespace AK.Scripts.Entities
{
    public class PlayerSquad : ITickable, IInitializable
    {
        private readonly List<Hero> _selectedHeroes = new();
        private Camera _camera;

        void IInitializable.Initialize()
        {
            _camera = Camera.main;
        }

        public Hero CurrentHero
        {
            get
            {
                if (_selectedHeroes.Count == 0)
                    return null;
                return _selectedHeroes[0];
            }
        }

        void ITickable.Tick()
        {
            if (_selectedHeroes.Count == 0)
                return;

            // bind camera
            const float cameraSpeed = 10;
            var firstHero = _selectedHeroes[0];
            var myPosition = firstHero.transform.position;
            var cameraTransform = _camera.transform;
            var cameraPosition = cameraTransform.position;
            var targetPosition = cameraPosition;
            targetPosition.x = myPosition.x;
            targetPosition.y = myPosition.y;
            var delta = targetPosition - cameraPosition;
            if (delta.magnitude <= cameraSpeed * Time.deltaTime)
            {
                cameraTransform.position = targetPosition;
                return;
            }
            var shift = delta.normalized * cameraSpeed * Time.deltaTime;
            cameraTransform.position = cameraPosition + shift;
        }

        public List<Hero> AllHeroes { get; } = new();

        public void CommandMove(Vector2 position)
        {
            // Vector2? firstHeroPosition = default;
            foreach (var hero in _selectedHeroes)
            {
                var targetPosition = position;
                // if (firstHeroPosition != null)
                // {
                //     var delta = (Vector2) hero.transform.position - firstHeroPosition.Value;
                //     targetPosition += delta;
                // }
                // else
                //     firstHeroPosition = hero.transform.position;

                hero.CommandMove(targetPosition);
            }
        }

        public void CommandAttack(Unit enemy)
        {
            foreach (var hero in _selectedHeroes)
            {
                hero.CommandAttack(enemy);
            }
        }

        public void SelectHero(Hero hero)
        {
            CleanupSelection();
            SelectHeroImpl(hero);
        }

        public void DeselectHero(Hero hero)
        {
            _selectedHeroes.Remove(hero);
            hero.HandleDeselect();
        }

        public void SelectAll()
        {
            foreach (var hero in AllHeroes)
            {
                if (!_selectedHeroes.Contains(hero))
                    SelectHeroImpl(hero);
            }
        }
        
        public void Select(int num)
        {
            if (num < AllHeroes.Count) 
                SelectHero(AllHeroes[num]);
        }

        private void SelectHeroImpl(Hero hero)
        {
            _selectedHeroes.Add(hero);
            hero.HandleSelect();
        }

        private void CleanupSelection()
        {
            foreach (var selected in _selectedHeroes)
                selected.HandleDeselect();
            _selectedHeroes.Clear();
        }
    }
}