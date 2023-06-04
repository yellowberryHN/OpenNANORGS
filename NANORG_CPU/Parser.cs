using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace OpenNANORGS.CPU
{
    public class Parser
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
                if (botInfo.Length < 2) throw new ArgumentException("Bot info header not formatted correctly");
                botName = botInfo[0].Trim();
                authorName = botInfo[1].Trim();
                lines.RemoveAt(0);
                Console.WriteLine($"name: {botName}\r\nauthor: {authorName}");
            }

            var lines_tmp = lines.ToArray();

            foreach (var line in lines_tmp)
            {
                if (line.Trim() == string.Empty || line.StartsWith("//") || line.StartsWith(";"))
                    lines.Remove(line);
            }

            ushort ip = 0; // instruction pointer
            ushort dp = 0; // data pointer (temporary)
            var labels = new Dictionary<string, ushort>();


            for (int j = 0; j < lines.Count; j++)
            {
                var line = lines[j];
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
                    if (dp % 3 == 0) ip = dp;
                    else if(dp > ip) ip += bufferedSize;
                }
                else if (l.EndsWith(":")) // label
                {
                    if (lines[j + 1].Split("//")[0].Split(';')[0].Trim().Split(null, 2)[0] == "data")
                        labels.Add(l.Split(":")[0].ToLower(), dp);
                    else
                        labels.Add(l.Split(":")[0].ToLower(), ip);
                }
                else
                {
                    ip += 3;
                    dp = ip;
                }
            }

            ip = 0;
            dp = 0;

            for (var j = 0; j < lines.Count; j++)
            {
                var line = lines[j];
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
                        if (!ushort.TryParse(_values[i], out dataArray[i]))
                        {
                            try
                            {
                                dataArray[i] = Convert.ToUInt16(_values[i], 16);
                            }
                            catch(FormatException)
                            {
                                if (labels.ContainsKey(_values[i].ToLower()))
                                    dataArray[i] = labels[_values[i].ToLower()];
                            }
                        }
                    }
                    
                    Buffer.BlockCopy(dataArray, 0, bytecode, (ip == dp ? ip * 2 : dp * 2), dataArray.Length * 2);
                    
                    var bufferedSize = (ushort)(Math.Ceiling(dataArray.Length / 3.0) * 3.0);
                    dp += (ushort)dataArray.Length;
                    if (dp % 3 == 0) ip = dp;
                    else if(dp >= ip) ip += bufferedSize;
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
                            op1type = CPUOperType.Immediate;
                            op1value = labels[op1s.ToLower()];
                        }
                        else if (op1s.StartsWith('r') && char.IsDigit(op1s[1]))
                        {
                            op1type = CPUOperType.Register;
                            op1value = ushort.Parse(op1s[1..]);
                        }
                        else if (op1s.StartsWith('['))
                        {
                            if (!char.IsDigit(op1s[2]) && op1s.Contains('+'))
                            {
                                // probably a label+register
                                op1type = CPUOperType.RegisterIndexed;
                                var tmp = op1s[1..^1].Split('+');
                                if (labels.ContainsKey(tmp[0].ToLower()) && tmp[1].StartsWith('r'))
                                {
                                    op1offset = labels[tmp[0].ToLower()];
                                    op1value = ushort.Parse(tmp[1][1..]);
                                }
                            }
                            else if (op1s.StartsWith("[r") && char.IsDigit(op1s[2]))
                            {
                                op1type = CPUOperType.RegisterIndexed;
                                if (!ushort.TryParse(op1s[2..^1], out op1value))
                                {
                                    if (op1s[2..^1].Contains('+'))
                                    {
                                        var tmp = op1s[2..^1].Split('+');
                                        op1value = ushort.Parse(tmp[0]);
                                        if (!ushort.TryParse(tmp[1], out op1offset))
                                        {
                                            try
                                            {
                                                op1offset = Convert.ToUInt16(tmp[1], 16);
                                            }
                                            catch (FormatException)
                                            {
                                                if (labels.ContainsKey(tmp[1].ToLower()))
                                                    op1offset = labels[tmp[1].ToLower()];
                                            }
                                        }
                                    }
                                    else if (op1s[2..^1].Contains('-'))
                                    {
                                        var tmp = op1s[2..^1].Split('-');
                                        op1sub = true;
                                        op1value = ushort.Parse(tmp[0]);
                                        if (!ushort.TryParse(tmp[1], out op1offset))
                                        {
                                            try
                                            {
                                                op1offset = Convert.ToUInt16(tmp[1], 16);
                                            }
                                            catch (FormatException)
                                            {
                                                if (labels.ContainsKey(tmp[1].ToLower()))
                                                    op1offset = labels[tmp[1].ToLower()];
                                            }
                                        }
                                    }
                                }
                            }
                            else
                            {
                                op1type = CPUOperType.Direct;
                                if (!ushort.TryParse(op1s[1..^1], out op1value))
                                {
                                    try
                                    {
                                        op1value = Convert.ToUInt16(op1s.TrimEnd(']')[1..], 16);
                                    }
                                    catch(FormatException)
                                    {
                                        if (labels.ContainsKey(op1s[1..^1].ToLower()))
                                            op1value = labels[op1s[1..^1].ToLower()];
                                    }
                                }
                            }
                        }
                        else
                        {
                            op1type = CPUOperType.Immediate;
                            if (!ushort.TryParse(op1s, out op1value))
                            {
                                try
                                {
                                    op1value = Convert.ToUInt16(op1s, 16);
                                }
                                catch(FormatException)
                                {
                                    if (labels.ContainsKey(op1s.ToLower()))
                                        op1value = labels[op1s.ToLower()];
                                }
                            }
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
                            
                            if (labels.ContainsKey(op2s))
                            {
                                op2type = CPUOperType.Immediate;
                                op2value = labels[op2s.ToLower()];
                            }
                            else if (op2s.StartsWith('r') && char.IsDigit(op2s[1]))
                            {
                                op2type = CPUOperType.Register;
                                op2value = ushort.Parse(op2s[1..]);
                            }
                            else if (op2s.StartsWith('['))
                            {
                                if (!char.IsDigit(op2s[2]) && op2s.Contains('+'))
                                {
                                    // probably a label+register
                                    op2type = CPUOperType.RegisterIndexed;
                                    var tmp = op2s[1..^1].Split('+');
                                    if (labels.ContainsKey(tmp[0].ToLower()) && tmp[1].StartsWith('r'))
                                    {
                                        op2offset = labels[tmp[0].ToLower()];
                                        op2value = ushort.Parse(tmp[1][1..]);
                                    }
                                }
                                else if (op2s.StartsWith("[r") && char.IsDigit(op2s[2]))
                                {
                                    op2type = CPUOperType.RegisterIndexed;
                                    if (!ushort.TryParse(op2s[2..^1], out op2value))
                                    {
                                        if (op2s[2..^1].Contains('+'))
                                        {
                                            var tmp = op2s[2..^1].Split('+');
                                            op2value = ushort.Parse(tmp[0]);
                                            if (!ushort.TryParse(tmp[1], out op2offset))
                                            {
                                                try
                                                {
                                                    op2offset = Convert.ToUInt16(tmp[1], 16);
                                                }
                                                catch (FormatException)
                                                {
                                                    if (labels.ContainsKey(tmp[1].ToLower()))
                                                        op2offset = labels[tmp[1].ToLower()];
                                                }
                                            }
                                        }
                                        else if (op2s[2..^1].Contains('-'))
                                        {
                                            var tmp = op2s[2..^1].Split('-');
                                            op2sub = true;
                                            op2value = ushort.Parse(tmp[0]);
                                            if (!ushort.TryParse(tmp[1], out op2offset))
                                            {
                                                try
                                                {
                                                    op2offset = Convert.ToUInt16(tmp[1], 16);
                                                }
                                                catch (FormatException)
                                                {
                                                    if (labels.ContainsKey(tmp[1].ToLower()))
                                                        op2offset = labels[tmp[1].ToLower()];
                                                }
                                            }
                                        }
                                    }
                                }
                                else
                                {
                                    op2type = CPUOperType.Direct;
                                    if (!ushort.TryParse(op2s[1..^1], out op2value))
                                    {
                                        try
                                        {
                                            op2value = Convert.ToUInt16(op2s[1..^1], 16);
                                        }
                                        catch(FormatException)
                                        {
                                            if (labels.ContainsKey(op2s[1..^1].ToLower()))
                                                op2value = labels[op2s[1..^1].ToLower()];
                                        }
                                    }
                                }
                            }
                            else
                            {
                                op2type = CPUOperType.Immediate;
                                if (!ushort.TryParse(op2s, out op2value))
                                {
                                    try
                                    {
                                        op2value = Convert.ToUInt16(op2s, 16);
                                    }
                                    catch(FormatException)
                                    {
                                        if (labels.ContainsKey(op2s.ToLower()))
                                            op2value = labels[op2s.ToLower()];
                                    }
                                }
                            }
                            op2 = new Operand(op2type, op2value, op2offset, op2sub);
                        }
                    }
                    
                    
                    //Console.Write("\n");

                    var inst = new Instruction(opcode, op1, op2, ip);
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
