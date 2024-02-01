using System;
using SadConsole;
using SadConsole.Input;
using SadConsole.Components;

namespace OpenNANORGS
{
    internal class RootScreen : ScreenObject
    {
        private Tank _tank;
        private Debugger _debugger;
        private Timer _timer;
        
        public RootScreen()
        {
            _tank = new Tank();
            Children.Add(_tank);
            
            /*
            _debugger = new Debugger();
            Children.Add(_debugger);
            _tank.EnableDebug(_debugger);
            */

            //_tank.SimulateSilent(999000);
            
            _timer = new Timer(System.TimeSpan.FromMilliseconds(2)) { Repeat = true };
            _timer.TimerElapsed += TimerCall;
            _timer.Start();
        }

        private void TimerCall(object? sender, EventArgs e)
        {
            if (_tank.Finished)
            {
                _timer.Stop();
                return;
            }
            _tank.Tick();
        }

        public override bool ProcessKeyboard(Keyboard keyboard)
        {
            bool handled = false;

            if (keyboard.IsKeyPressed(Keys.Enter))
            {
                _timer.Stop();
                handled = true;
            }

            return handled;
        }

        public override void Update(TimeSpan delta)
        {
            _timer.Update(this, delta);
            base.Update(delta);
        }
    }
}