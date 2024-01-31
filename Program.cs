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
            Settings.WindowTitle = "SadConsole Examples";

            // Configure how SadConsole starts up
            Builder startup = new Builder()
                    .SetScreenSize(90, 30)
                    .UseDefaultConsole()
                    .OnStart(Game_Started)
                    .IsStartingScreenFocused(true)
                    .ConfigureFonts((config, game) => config.UseBuiltinFontExtended())
                ;

            // Setup the engine and start the game
            Game.Create(startup);
            Game.Instance.Run();
            Game.Instance.Dispose();
        }
        
        static void Game_Started(object? sender, GameHost host)
        {
            ColoredGlyph boxBorder = new(Color.White, Color.Black, 178);
            ColoredGlyph boxFill = new(Color.White, Color.Black);

            Game.Instance.StartingConsole.FillWithRandomGarbage(255);
            Game.Instance.StartingConsole.DrawBox(new Rectangle(2, 2, 26, 5), ShapeParameters.CreateFilled(boxBorder, boxFill));
            Game.Instance.StartingConsole.Print(4, 4, "Welcome to SadConsole!");
        }
    }
}