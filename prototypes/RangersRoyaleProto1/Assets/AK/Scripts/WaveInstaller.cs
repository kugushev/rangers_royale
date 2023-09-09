using Zenject;

namespace AK.Scripts
{
    public class WaveInstaller: MonoInstaller
    {
        public override void InstallBindings()
        {
            Container.Bind<WaveManager>().FromComponentInHierarchy().AsSingle();
        }
    }
}