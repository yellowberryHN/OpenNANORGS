using System;
using Console = SadConsole.Console;
using SadConsole;
using SadConsole.Configuration;
using SadRogue.Primitives;

namespace OpenNANORGS
{
    class Program
    {
        [STAThread]
        static void Main(string[] args)
        {
            Settings.WindowTitle = "OpenNANORGS";

            // Configure how SadConsole starts up
            Builder startup = new Builder()
                    .SetScreenSize(80, 50)
                    .SetStartingScreen<RootScreen>()
                    .IsStartingScreenFocused(true)
                    .ConfigureFonts((config, game) => config.UseBuiltinFontExtended())
                ;

            // Setup the engine and start the game
            Game.Create(startup);
            Game.Instance.Run();
            Game.Instance.Dispose();
        }
    }
}