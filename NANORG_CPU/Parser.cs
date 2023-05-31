using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace NANORG_CPU
{
    class Parser
    {
        public List<Instruction> instructionList;
        public ushort[] bytecode = new ushort[3600];

        public string botName;
        public string authorName;

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
                var botInfo = lines[0].Split(':')[1].Split(',', 2);
                botName = botInfo[0].Trim();
                authorName = botInfo[1].Trim();
                lines.RemoveAt(0);
                Console.WriteLine($"name: {botName}\r\nauthor: {authorName}");
            }

            ushort ip = 0; // instruction pointer
            ushort dp = 0; // data pointer (temporary)
            var labels = new Dictionary<string, ushort>();

            foreach (var line in lines)
            {
                var l = line.Split("//")[0].Split(';')[0].Trim();
                if (l == string.Empty || l.StartsWith("//") || l.StartsWith(";")) continue;
                var op = l.Split(null, 2);
                
                if(op[0] == "data")
                {
                    var data = l.Split(null, 2)[1];

                    if (!data.StartsWith('{')) throw new ArgumentException("invalid call to data function");
                    var _values = data.TrimEnd('}')[1..].Trim().Split(null);
                    
                    var bufferedSize = (ushort)(Math.Ceiling(_values.Length / 3.0) * 3.0);
                    dp += (ushort)_values.Length;
                    if(dp >= ip) ip += bufferedSize;
                }
                else if (l.EndsWith(":")) // label
                {
                    labels.Add(line.Split(":")[0], dp == ip ? ip : dp);
                }
                else
                {
                    ip += 3;
                    dp = ip;
                }
            }

            ip = 0;
            dp = 0;

            foreach (var line in lines)
            {
                
                var l = line.Split("//")[0].Split(';')[0].Trim();
                if (l == string.Empty || l.StartsWith("//") || l.StartsWith(";") || l.EndsWith(":")) continue;

                //Console.WriteLine($"ip: {ip}, dp: {dp}");
                
                var op = l.Split(null, 2);
                
                if(op[0] == "data")
                {
                    var data = l.Split(null, 2)[1];
                    //Console.WriteLine($"data detected: {data}");

                    if (!data.StartsWith('{')) throw new ArgumentException("invalid call to data function");
                    var _values = data.TrimEnd('}')[1..].Trim().Split(null);
                    var dataArray = new ushort[_values.Length];

                    for (int i = 0; i < _values.Length; i++)
                    {
                        // TODO: this will break for labels
                        if (!ushort.TryParse(_values[i], out dataArray[i]))
                            dataArray[i] = Convert.ToUInt16(_values[i], 16);
                    }
                    
                    Buffer.BlockCopy(dataArray, 0, bytecode, (ip == dp ? ip * 2 : dp * 2), dataArray.Length * 2);
                    
                    var bufferedSize = (ushort)(Math.Ceiling(dataArray.Length / 3.0) * 3.0);
                    dp += (ushort)dataArray.Length;
                    if(dp >= ip) ip += bufferedSize;
                }
                else
                {
                    string[] operands = null;
                    if (op.Length > 1) 
                        operands = op[1].Split(',');

                    var opcode = Enum.Parse<CPUOpCode>(op[0], true);
                    Operand op1 = null;
                    Operand op2 = null;

                    //Console.Write($"op: {op[0]}");
                    if (operands != null)
                    {
                        var op1s = operands[0].Trim();
                        var op1type = CPUOperType.Direct;
                        ushort op1value = 0;
                        ushort op1offset = 0;
                        var op1sub = false;
                        
                        //Console.Write($", op1: {op1s}");

                        if (labels.ContainsKey(op1s))
                        {
                            // TODO: compensate for jumps and calls
                            op1type = CPUOperType.Immediate;
                            op1value = labels[op1s];
                        }
                        else if (op1s.StartsWith("r"))
                        {
                            op1type = CPUOperType.Register;
                            op1value = ushort.Parse(op1s[1..]);
                        }
                        else if (op1s.StartsWith("["))
                        {
                            if (op1s.StartsWith("[r"))
                            {
                                // TODO: Compensate for offset
                                op1type = CPUOperType.RegisterIndexed;
                                op1value = ushort.Parse(op1s.TrimEnd(']')[2..]);
                            }
                            else
                            {
                                // TODO: compensate for jumps and calls
                                op1type = CPUOperType.Direct;
                                if (!ushort.TryParse(op1s.TrimEnd(']')[1..], out op1value) &&
                                    labels.ContainsKey(op1s.TrimEnd(']')[1..]))
                                    op1value = labels[op1s.TrimEnd(']')[1..]];
                            }
                        }
                        else
                        {
                            op1type = CPUOperType.Immediate;
                            if (!ushort.TryParse(op1s, out op1value))
                                op1value = Convert.ToUInt16(op1s, 16);
                        }
                        op1 = new Operand(op1type, op1value, op1offset, op1sub);
                        
                        if (operands.Length == 2)
                        {
                            var op2s = operands[1].Trim();
                            var op2type = CPUOperType.Direct;
                            ushort op2value = 0;
                            ushort op2offset = 0;
                            var op2sub = false;
                            
                            //Console.Write($", op2: {op2s}");
                            
                            if (op2s.StartsWith("r"))
                            {
                                op2type = CPUOperType.Register;
                                op2value = ushort.Parse(op2s[1..]);
                            }
                            else if (op2s.StartsWith("["))
                            {
                                if (op2s.StartsWith("[r"))
                                {
                                    // TODO: Compensate for offset
                                    op2type = CPUOperType.RegisterIndexed;
                                    op2value = ushort.Parse(op2s.TrimEnd(']')[2..]);
                                }
                                else
                                {
                                    op2type = CPUOperType.Direct;
                                    op2value = ushort.Parse(op2s.TrimEnd(']')[1..]);
                                }
                            }
                            else
                            {
                                op2type = CPUOperType.Immediate;
                                if (!ushort.TryParse(op2s, out op2value))
                                    op2value = Convert.ToUInt16(op2s, 16);
                            }
                            op2 = new Operand(op2type, op2value, op2offset, op2sub);
                        }
                    }
                    
                    
                    //Console.Write("\n");

                    var inst = new Instruction(opcode, op1, op2);
                    Console.WriteLine($"{ip:D4} {inst}");
                    
                    Buffer.BlockCopy(inst.bytecode, 0, bytecode, ip * 2, inst.bytecode.Length * 2);
                    
                    tmpList.Add(inst);
                    ip += 3; // for labels and jumps
                    dp = ip;
                }
                //Console.WriteLine($"new ip: {ip}, new dp: {dp}");
            }
            
            return tmpList;
        }
    }
}
