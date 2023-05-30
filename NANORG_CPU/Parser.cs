using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace NANORG_CPU
{
    class Parser
    {
        public List<Instruction> instructionList;

        public Parser(string filename)
        {
            instructionList = Load(filename);
        }

        private List<Instruction> Load(string filename)
        {
            var tmpList = new List<Instruction>();

            var lines = File.ReadLines(filename).ToList();

            if (lines[0].StartsWith("info:"))
            {
                lines.RemoveAt(0);
            }

            ushort ip = 0; // instruction pointer
            var labels = new Dictionary<ushort, string>();

            foreach (var line in lines)
            {
                var l = line.Trim();
                if (l == String.Empty) continue;
                if (l.EndsWith(":")) // label
                {
                    labels.Add(ip, line.Split(":")[0]);
                    continue;
                }

                var op = l.Split(null, 2);
                string[] operands = null;
                if (op.Length > 1) 
                    operands = op[1].Split(',');

                CPUOpCode opcode;
                Operand op1;
                Operand op2;

                Console.Write($"op: {op[0]}");
                opcode = Enum.Parse<CPUOpCode>(op[0], true);
                if (operands != null)
                {
                    Console.Write($", op1: {operands[0].Trim()}");
                    op1 = new Operand(); // TODO: make this do things
                    if (operands.Length == 2)
                    {
                        Console.Write($", op2: {operands[1].Trim()}");
                        op2 = new Operand();
                    }
                }
                Console.Write("\n");
            }
            
            return tmpList;
        }
    }
}
