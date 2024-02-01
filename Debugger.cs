using SadConsole;

namespace OpenNANORGS
{
    internal class Debugger : ScreenObject
    {
        private Console _console;

        public Bot? Bot { get; private set; }

        public Debugger()
        {
            _console = new Console(70, 5);
            _console.Position = (0, 43);
            //_console.Surface.DefaultBackground = Color.AnsiGreen;

            Children.Add(_console);

            FillTempInfo(_console);
        }

        private void FillTempInfo(Console console)
        {
            console.Print(0, 0, "(AJOSA A) x=45, y=25, energy=10000, IP=0, SP=3600, flags=");
            console.Print(0, 1, "R00=    0 R01=    0 R02=    0 R03=    0 R04=    0 R05=    0 R06=    0");
            console.Print(0, 2, "R07=    0 R08=    0 R09=    0 R10=    0 R11=    0 R12=    0 R13=    0");
            console.Print(0, 3, "0000  travel 0");
            console.Print(0, 4, "(u)nasm,(g)o,(s)ilentGo,(d)mp,(e)dt,(r)eg,(i)p,(q)uit,##, or [Enter]:");

            /*
            (AJOSA A) x=45, y=25, energy=10000, IP=0, SP=3600, flags=
            R00=    0 R01=    0 R02=    0 R03=    0 R04=    0 R05=    0 R06=    0 
            R07=    0 R08=    0 R09=    0 R10=    0 R11=    0 R12=    0 R13=    0
            0000  travel 0
            (u)nasm,(g)o,(s)ilentGo,(d)mp,(e)dt,(r)eg,(i)p,(q)uit,##, or [Enter]:
            */
        }

        public void SetBot(Bot bot)
        {
            Bot = bot;
        }
    }
}