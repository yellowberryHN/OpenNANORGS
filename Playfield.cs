using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace OpenNANORGS
{
    class Playfield
    {
        // hold data about how many sludge types, which are toxic, where collection points are located,
        // where to spawn nanorgs and drones, where all items are on the map.

        private ushort[,] elements = new ushort[40, 70];

        private byte numSludge;
        private List<byte> toxicSludge;
        private bool debugSludge = false;

        private Random rnd;

        private uint tick = 0;

        private ulong score = 0;

        

        private Bot testBot;

        private List<Bot> bots;

        public Playfield(int seed)
        {
            rnd = new Random(seed);
            numSludge = (byte)rnd.Next(5, 32);

            toxicSludge = new List<byte> { };

            for (int i = 0; i < (byte)(numSludge * 0.2); i++)
            {
                toxicSludge.Add((byte)rnd.Next(1, numSludge));
            }

            Randomize();



            testBot = new Bot(seed, 'A', 69, 39, this);
        }

        public StringBuilder Tick()
        {
            testBot.Tick();
            var sb = Render();
            tick++;
            return sb;
        }

        // check to see bot can move to specific tile
        public bool Occupied(int x, int y)
        {
            if (x > 69 || y > 39 || x < 0 || y < 0) return true;

            // TODO: change when all bots are added
            if (x == testBot.x && y == testBot.y)
            {
                return true;
            }

            return false;
        }

        public bool IsToxic(ushort id)
        {
            return toxicSludge.Contains((byte)id);
        }

        public void Randomize()
        {
            elements = new ushort[40, 70];
            for (int i = 0; i < 200; i++)
            {
                bool suc = false;
                while (suc == false)
                {
                    suc = SetElement((byte)rnd.Next(70), (byte)rnd.Next(40), (ushort)rnd.Next(1, numSludge));
                }
            }

            for (int i = 0; i < 10; i++)
            {
                bool suc = false;
                while (suc == false)
                {
                    suc = SetElement((byte)rnd.Next(70), (byte)rnd.Next(40), 65535);
                }
            }
        }

        public ushort GetElement(byte x, byte y)
        {
            return elements[y, x];
        }

        public ushort Consume(Bot bot)
        {
            var sludge = elements[bot.y, bot.x];
            if (sludge != 65535 && sludge != 0)
            {
                elements[bot.y, bot.x] = 0;
                bool suc = false;
                while (suc == false)
                {
                    suc = SetElement((byte)rnd.Next(70), (byte)rnd.Next(40), sludge);
                }
            }
            else if (sludge == 65535) sludge = 0;
            return sludge;
        }

        public void Collect(Bot bot, ushort amt)
        {
            if(elements[bot.y, bot.x] == 65535)
            {
                score += amt;
            }
        }

        private bool SetElement(byte x, byte y, ushort id)
        {
            if (elements[y, x] != 0) return false;
            elements[y, x] = id;
            return true;
        }

        public StringBuilder Render()
        {
            var sb = new StringBuilder();

            for (int y = 0; y < 40; y++)
            {
                string tmp = string.Empty;
                for (int x = 0; x < 70; x++)
                {
                    if(testBot.x == x && testBot.y == y)
                    {
                        if (elements[y, x] > 0)
                        {
                            tmp += testBot.Render().ToString().ToLower();
                        }
                        else
                        {
                            tmp += testBot.Render();
                        }
                        
                    }
                    else
                    {
                        switch (elements[y, x])
                        {
                            case 0:
                                tmp += " ";
                                break;
                            case 65535:
                                tmp += "$";
                                break;
                            default:
                                if(toxicSludge.Contains((byte)elements[y, x]))
                                {
                                    tmp += "%";
                                }
                                else
                                {
                                    tmp += debugSludge ? elements[y, x].ToString() : "*";
                                }
                                
                                break;
                        }
                    }
                }
                sb.AppendLine(tmp);
            }


            sb.AppendLine(string.Format("\r\nScore: {0}, Ticks: {1} of 1,000,000", score, tick));

            sb.AppendLine(string.Format("\r\nX: {0:D2}, Y: {1:D2}, Energy: {2:D5}", testBot.x, testBot.y, testBot.energy));

            sb.AppendLine(string.Format("\r\n"));

            /*
            Score: 0, Ticks: 0 of 1000000   (Seed=1641076912)

            (AJOSA A) x=45, y=25, energy=10000, IP=0, SP=3600, flags=
            R00=    0 R01=    0 R02=    0 R03=    0 R04=    0 R05=    0 R06=    0
            R07=    0 R08=    0 R09=    0 R10=    0 R11=    0 R12=    0 R13=    0
            0000  travel 0
            (u)nasm,(g)o,(s)ilentGo,(d)mp,(e)dt,(r)eg,(i)p,(q)uit,##, or [Enter]:
            */


            return sb;
        }
    }
}
