using AK.Scripts.Entities;
using AK.Scripts.Services;
using Zenject;

namespace AK.Scripts
{
    public class PlaygroundInstaller: MonoInstaller
    {
        public override void InstallBindings()
        {
            Container.BindInterfacesAndSelfTo<InputService>().AsSingle();
            Container.BindInterfacesAndSelfTo<PlayerSquad>().AsSingle();
            Container.BindInterfacesAndSelfTo<PauseService>().AsSingle();
            Container.Bind<Bonfire>().FromComponentInHierarchy().AsSingle();
            Container.Bind<World>().FromComponentInHierarchy().AsSingle();
        }
    }
}