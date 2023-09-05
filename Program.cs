using System;
using System.Diagnostics;
using System.Threading;

namespace OpenNANORGS
{
    class Program
    {
        protected static void WriteAt(string s, int x, int y)
        {
            try
            {
                Console.SetCursorPosition(x, y);
                Console.Write(s);
            }
            catch (ArgumentOutOfRangeException e)
            {
                Console.Clear();
                Console.WriteLine(e.Message);
            }
        }

        public static void Main(string[] args)
        {
            if (OperatingSystem.IsWindows()) // Attempt native resize on Windows
            {
                Console.SetWindowSize(1, 1); // done to be able to set the buffer to the correct size
                Console.SetBufferSize(80, 50);
                Console.SetWindowSize(80, 50);
            }
            else // Attempt ANSI escape resize otherwise
            {
                Console.Write("\x1b[8;50;70t");
            }

            Console.Clear();

            var pf = new Tank(args);
            uint tick = 0;

            while (!pf.Finished())
            {
                if (!pf.debugBot) Console.CursorVisible = false;
                
                if (!pf.quiet && ((tick % 10) == 0 || pf.debugBot))
                {
                    pf.Render();
                    Console.Write(pf.builder);
                    if (pf.debugBot)
                    {
                        pf.DebugHighlight();
                        pf.DebugControl(Console.ReadLine());
                    }
                    else Thread.Sleep(10);
                }
                tick = pf.Tick();
                if(tick % 1000 == 0) Console.Write(tick / 1000); 
                Console.SetCursorPosition(0, 0);
            }

            Console.Clear();
            
            //Console.Write(pf.);
            Console.WriteLine("done.");
        }
    }
}
