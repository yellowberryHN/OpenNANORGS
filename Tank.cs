using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using SadConsole;
using SadRogue.Primitives;

namespace OpenNANORGS
{
    internal class Tank : ScreenObject
    {
        // hold data about how many sludge types, which are toxic, where collection points are located,
        // where to spawn nanorgs and drones, where all items are on the map.

        // new things
        
        private ScreenSurface _map;
        private ScreenSurface _score;
        private StringBuilder _scoreBuilder = new();

        private List<Bot> _bots = new();
        
        private Bot? _debugBot;
        private Debugger _debugger;
        private char debugBotID = 'm';

        private ushort[,] elements;
        
        private byte numSludge;
        private List<ushort> toxicSludge;
        private bool debugSludge = false;
        
        public int seed;
        public Random rnd;

        public uint currentTick { get; private set; } = 0;
        public uint maxTicks { get; } = 1000000;

        private ulong score = 0;
        
        public ScreenSurface Map => _map;
        public bool Finished => currentTick >= maxTicks;
        
        // old things
        
        private string botSource = string.Empty;


        //private Parser botAssembly;

        public bool quiet;

        // does the tank loop...
        private bool loopH; // horizontally?
        private bool loopV; // vertically?

        public Tank(int width = 70, int height = 40)
        {
            _map = new ScreenSurface(width, height);
            _map.UseMouse = false;
            //_map.Surface.DefaultBackground = Color.AnsiRed;

            Children.Add(_map);

            _score = new ScreenSurface(width, 3);
            _score.Position = (0, height);
            _score.UseMouse = false;
            //_score.Surface.DefaultBackground = Color.AnsiBlue;
            
            Children.Add(_score);
            
            seed = (int)DateTimeOffset.Now.ToUnixTimeSeconds();

            elements = new ushort[height, width];
            // TODO: System.Random is not reliable across different platforms and .NET versions
            rnd = new Random(this.seed); 
            numSludge = byte.Max((byte)(GetNextRand(0, 255)/8), 0x5);

            toxicSludge = new List<ushort>();

            for (int i = 0; i < (byte)(numSludge * 0.2); i++)
            {
                toxicSludge.Add((byte)GetNextRand(1, numSludge));
            }

            Randomize();
            RenderMap();

            //botAssembly = new Parser(botSource);

            byte rndX = 0, rndY = 0;

            for (var i = 0; i < 26; i++)
            {
                while (IsOccupied((rndX, rndY)))
                {
                    rndX = (byte)GetNextRand(_map.Width);
                    rndY = (byte)GetNextRand(_map.Height);
                }
                var bot = new Bot((char)(65 + i), (rndX, rndY), this);
                _bots.Add(bot);
            }
            for (var i = 0; i < 24; i++) // only 50 bots, poor 'y' and 'z' :(
            {
                while (IsOccupied((rndX, rndY)))
                {
                    rndX = (byte)GetNextRand(_map.Width);
                    rndY = (byte)GetNextRand(_map.Height);
                }
                var bot = new Bot((char)(97 + i), (rndX, rndY), this);
                _bots.Add(bot);
            }
            for (var i = 0; i < 20; i++)
            {
                while (IsOccupied((rndX, rndY)))
                {
                    rndX = (byte)GetNextRand(_map.Width);
                    rndY = (byte)GetNextRand(_map.Height);
                }
                var drone = new Drone((rndX, rndY), this);
                _bots.Add(drone);
            }
        }

        public int GetNextRand(int minValue, int maxValue)
        {
            return rnd.Next(minValue, maxValue);
        }
        
        public int GetNextRand(int maxValue)
        {
            return GetNextRand(0, maxValue);
        }

        public void EnableDebug(Debugger debugger)
        {
            _debugger = debugger;
            _debugBot = _bots.Find(x => x.Name == debugBotID);

            if (_debugBot != null) debugger.SetBot(_debugBot);
        }

        /*
        public void DebugHighlight()
        {
            // user requested no color in terminal output
            if (Environment.GetEnvironmentVariable("NO_COLOR") != null) return;
            
            var (tmpLeft, tmpTop) = Console.GetCursorPosition();
            
            if (OperatingSystem.IsWindows())
            {
                Console.SetCursorPosition(dBI.x, dBI.y);
                Console.ForegroundColor = ConsoleColor.Red;
                Console.Write(dBI.Render());
                Console.ResetColor();
                Console.SetCursorPosition(tmpLeft, tmpTop);
            }
            else
            {
                Console.Write($"\x1b[{dBI.x};{dBI.y}H");
                Console.Write("\x1b[31m");
                Console.Write(dBI.Render());
                Console.Write("\x1b[0m");
                Console.Write($"\x1b[{tmpLeft};{tmpTop}H");
            }
        }
        */

        /*
        private void ParseArgs(string[] args)
        {
            try
            {
                foreach (var item in args)
                {
                    if (item.Length >= 2 && item.StartsWith('-'))
                    {
                        var flag = item.Substring(1, 1);
                        var param = item.Contains(':') ? item[3..] : string.Empty;
                        switch (flag)
                        {
                            case "p":
                                botSource = param;
                                break;
                            case "s":
                                if (!int.TryParse(param, out seed))
                                {
                                    throw new ArgumentException("Invalid seed!");
                                }

                                break;
                            case "q":
                                quiet = true;
                                break;
                            case "g":
                                debugBot = true;
                                debugBotID = param.ToCharArray()[0];
                                if (debugBotID == 'y' || debugBotID == 'z' || !Char.IsAsciiLetter(debugBotID))
                                    throw new ArgumentException("invalid debug ID specified");
                                break;
                            case "i":
                                if (!uint.TryParse(param, out maxTicks))
                                {
                                    throw new ArgumentException("invalid iteration count");
                                }
                                break;
                            case "w":
                                if (!ushort.TryParse(param, out width))
                                {
                                    throw new ArgumentException("invalid width");
                                }
                                break;
                            case "h":
                                if (!ushort.TryParse(param, out height))
                                {
                                    throw new ArgumentException("invalid height");
                                }
                                break;
                            default:
                                throw new ArgumentException($"invalid parameter (-{flag})");
                        }
                    }
                    else throw new ArgumentException($"invalid parameter ({item})");
                }

                if (args.Length == 0)
                {

                }
            }
            catch (ArgumentException e)
            {
                Console.WriteLine("Error loading game:");
                Console.WriteLine($" {e.Message}");
                Environment.Exit(1);
            }
            
        }
        */

        public uint Tick()
        {
            foreach (var bot in _bots)
            {
                bot.Tick();
            }
            
            currentTick++;
            return currentTick;
        }

        public override void Update(TimeSpan delta)
        {
            _score.Print(0, 1, $"Score: {score}, Ticks: {currentTick} of {maxTicks}, Seed: <{seed}>");
            
            base.Update(delta);
        }

        public void SimulateSilent(uint ticks)
        {
            while (ticks > currentTick)
            {
                Tick();
            }
        }
        
        public bool IsToxic(ushort id)
        {
            return toxicSludge.Contains((byte)id);
        }

        public void Randomize()
        {
            _map.Clear();

            elements = new ushort[_map.Height,_map.Width];
            
            for (int i = 0; i < GetNextRand(100, 200); i++)
            {
                bool suc = false;
                while (suc == false)
                {
                    suc = SetElement((byte)GetNextRand(_map.Width), (byte)GetNextRand(_map.Height), (ushort)GetNextRand(1, numSludge));
                }
            }

            for (int i = 0; i < 10; i++)
            {
                bool suc = false;
                while (suc == false)
                {
                    suc = SetElement((byte)GetNextRand(_map.Width), (byte)GetNextRand(_map.Height), 0xFFFF);
                }
            }
        }

        public ushort GetElement(int x, int y)
        {
            return elements[y, x];
        }
        
        private bool SetElement(int x, int y, ushort id)
        {
            if (elements[y, x] != 0) return false;
            elements[y, x] = id;
            return true;
        }
        
        public ushort Consume(Bot bot)
        {
            var sludge = elements[bot.Y, bot.X];
            if (sludge != 0xFFFF && sludge != 0)
            {
                elements[bot.Y, bot.X] = 0;
                var suc = false;
                byte rndX = 0, rndY = 0;
                while (!suc)
                {
                    rndX = (byte)GetNextRand(_map.Width);
                    rndY = (byte)GetNextRand(_map.Height);
                    suc = SetElement(rndX, rndY, sludge);
                }
                RestoreMapGlyph(rndX, rndY);
            }
            else if (sludge == 0xFFFF) sludge = 0; // on a collection point, report back that nothing was consumed

            if (IsToxic(sludge)) bot.Mutate();
            return sludge;
        }
        
        /*
        public bool Collect(Bot bot, ushort amt)
        {
            if (elements[bot.y, bot.x] != 0xFFFF) return false;
            bot.energy -= amt;
            score += amt;
            return true;
        }

        public bool PokeBot(Bot bot, ushort dir, ushort offset)
        {
            int x = bot.x;
            int y = bot.y;
            
            switch (dir % 4)
            {
                case 0:
                    y--;
                    break;
                case 1:
                    y++;
                    break;
                case 2:
                    x++;
                    break;
                case 3: 
                    x--;
                    break;
            }
            
            var poked = bots.Find(b => b.x == x && b.y == y);
            if (poked == null) return false;

            poked.cpu.PMemory[offset] = bot.cpu.Registers[0];
            return true;
        }
        
        public int PeekBot(Bot bot, ushort dir, ushort offset)
        {
            int x = bot.x;
            int y = bot.y;
            
            switch (dir % 4)
            {
                case 0:
                    y--;
                    break;
                case 1:
                    y++;
                    break;
                case 2:
                    x++;
                    break;
                case 3: 
                    x--;
                    break;
            }

            var poked = bots.Find(b => b.x == x && b.y == y);
            if (poked == null) return -1;

            var peeked = poked.cpu.PMemory[offset];
            return peeked;
        }
        
        public bool ChargeBot(Bot bot, ushort dir, ushort amount)
        {
            int x = bot.x;
            int y = bot.y;
            
            switch (dir % 4)
            {
                case 0:
                    y--;
                    break;
                case 1:
                    y++;
                    break;
                case 2:
                    x++;
                    break;
                case 3: 
                    x--;
                    break;
            }

            if (bot.energy < amount + 1) return false;

            var poked = bots.Find(b => b.x == x && b.y == y);
            if (poked == null) return false;
            if (poked.energy + amount > 0xFFFF) return false;

            bot.energy -= amount;
            poked.energy += amount;

            return true;
        }
        
        // check to see bot can move in a specific direction
        public bool ValidMove(Bot bot, int dir)
        {
            int x = bot.x;
            int y = bot.y;
            
            switch (dir % 4)
            {
                case 0:
                    y--;
                    break;
                case 1:
                    y++;
                    break;
                case 2:
                    x++;
                    break;
                case 3: 
                    x--;
                    break;
            }
            
            if ((x > width - 1 || x < 0) && !loopH || (y > height - 1 || y < 0 ) && !loopV) return false;
            if (loopH) x = (byte)(x % width);
            if (loopV) y = (byte)(y % height);
            
            return bots.Find(b => b.x == x && b.y == y) == null;
        }
        
        */

        /*
        public void RenderMap()
        {
            builder.Clear();
            for (var y = 0; y < _map.Height; y++)
            {
                for (var x = 0; x < _map.Width; x++)
                {
                    var occ = false; // block already drawn.
                    /*
                    foreach (var bot in bots)
                    {
                        if (bot.x != x || bot.y != y) continue;
                        builder.Append(bot.Render());
                        occ = true;
                    }
                    if (!occ)
                    {
                        switch (elements[y, x])
                        {
                            case 0: // empty tile
                                builder.Append(' ');
                                break;
                            case 65535: // collection point
                                builder.Append('$');
                                break;
                            default:
                                if(toxicSludge.Contains((byte)elements[y, x]))
                                {
                                    builder.Append(debugSludge ? '%' : '*');
                                }
                                else
                                {
                                    builder.Append(debugSludge ? elements[y, x].ToString()[0]: '*');
                                }
                                break;
                        }
                        
                        switch (elements[y, x])
                        {
                            case 0:
                                break;
                            case 65535:
                                _map.SetGlyph(x, y, '$');
                                break;
                            default:
                                _map.SetGlyph(x, y, '*');
                                break;
                        }
                    }
                }

                builder.Append(Environment.NewLine);
            }

            builder.AppendLine($"{Environment.NewLine}Score: {score:n0}, Ticks: {tick:n0} of {maxTicks:n0}, Seed: <{seed}>");

            //sb.AppendLine(string.Format("\r\nX: {0:D2}, Y: {1:D2}, Energy: {2:D5}", testBot.x, testBot.y, testBot.energy));

            /*
            if (debugBot) // TODO: the tank shouldn't be in charge of this, what the fuck
            {
                var botName = botAssembly.botName.ToUpper();
                builder.AppendLine(string.Format("\r\n({0} {1}) x={2}, y={3}, energy={4}, IP={5}, SP={6}, flags={7}", (botName.Length < 5 ? botName : botName.Substring(0 ,5)), dBI.botId, dBI.x, dBI.y, dBI.energy, dBI.ip, dBI.sp, dBI.cpu.FlagRender()));
                builder.AppendLine(string.Format("R00={0,5} R01={1,5} R02={2,5} R03={3,5} R04={4,5} R05={5,5} R06={6,5}", dBI.reg[0], dBI.reg[1], dBI.reg[2], dBI.reg[3], dBI.reg[4], dBI.reg[5], dBI.reg[6]));
                builder.AppendLine(string.Format("R07={0,5} R08={1,5} R09={2,5} R10={3,5} R11={4,5} R12={5,5} R13={6,5}", dBI.reg[7], dBI.reg[8], dBI.reg[9], dBI.reg[10], dBI.reg[11], dBI.reg[12], dBI.reg[13]));
                builder.AppendLine(dBI.cpu.Disassemble());
                builder.AppendLine($"mutate={dBI.cpu.mutations.Count}");
                builder.Append("(u)nasm,(g)o,(s)ilentGo,(d)mp,(e)dt,(r)eg,(i)p,(q)uit,##,or [Enter]: ");
            }

            /*
            (%s %c) x=%d, y=%d, energy=%d, IP=%d, SP=%d, flags=%s                 
            (u)nasm,(g)o,(s)ilentGo,(d)mp,(e)dt,(r)eg,(i)p,(q)uit,##, or [Enter]: 
            Score: %.0lf, Ticks: %d of %d   (Seed=%u)                             
            */

            /*
            Score: 0, Ticks: 0 of 1000000   (Seed=1641076912)

            (AJOSA A) x=45, y=25, energy=10000, IP=0, SP=3600, flags=
            R00=    0 R01=    0 R02=    0 R03=    0 R04=    0 R05=    0 R06=    0 
            R07=    0 R08=    0 R09=    0 R10=    0 R11=    0 R12=    0 R13=    0
            0000  travel 0
            (u)nasm,(g)o,(s)ilentGo,(d)mp,(e)dt,(r)eg,(i)p,(q)uit,##, or [Enter]:
            */
            
            /*
            Entrant: AJOSAMPLEBOT, JOHN DOE
            Your score: 491,310
            Live organisms: 8, Live drones: 20, Final tick #: 1000000, Seed: 1673077679
            
        }
        */

        public void RenderMap()
        {
            for (var y = 0; y < _map.Height; y++)
            {
                for (var x = 0; x < _map.Width; x++)
                {
                    RestoreMapGlyph(x,y);
                }
            }
        }

        public void RestoreMapGlyph(Point position)
        {
            RestoreMapGlyph(position.X, position.Y);
        }

        public void RestoreMapGlyph(int x, int y)
        {
            var element = GetElement(x, y);
            switch (element)
            {
                case 0:
                    _map.SetGlyph(x, y, 0, Color.White);
                    break;
                case 0xFFFF: // collection point
                    _map.SetGlyph(x, y, '$', Color.White);
                    break;
                default:
                    SetSludgeGlyph(element, x, y, _map);
                    break;
            }
        }

        private void SetSludgeGlyph(ushort sludge, int x, int y, IScreenSurface map)
        {
            Color glyphColor = Color.White;
            char glyph = '*';

            if (debugSludge && IsToxic(sludge)) glyphColor = Color.LimeGreen;
            if (debugSludge) glyph = sludge.ToString("X")[0];

            _map.SetGlyph(x, y, glyph, glyphColor);
        }

        public void RenderUpdate()
        {
            // empty for now
        }

        public bool IsOccupied(Point newPosition)
        {
            if (!_map.Surface.IsValidCell(newPosition.X, newPosition.Y)) return true;
            if (_bots.Any(bot => bot.Position == newPosition)) return true;
            
            return false;
        }
    }
}
