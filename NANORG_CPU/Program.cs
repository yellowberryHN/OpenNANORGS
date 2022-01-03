using System;

namespace NANORG_CPU
{
    class Program
    {
        static void Main(string[] args)
        {

            // make sure to account for relative positioning on jumps and calls (cringe)

            /*
            if ((opcode >= CPUOpCode.JMP && opcode <= CPUOpCode.JNS) || opcode == CPUOpCode.CALL)
            {

            }
            */

            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Register, 5), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Register, 5), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Direct, 5), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 5, 255), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 5, 255, true), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 9, 12), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Direct, 12), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Register, 3), new Operand(CPUOperType.Register, 4)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 4, 7), new Operand(CPUOperType.RegisterIndexed, 4, 7)));
            Console.WriteLine(new Instruction(CPUOpCode.ADD, new Operand(CPUOperType.Register, 6), new Operand(CPUOperType.RegisterIndexed, 4, 7, true)));
            Console.WriteLine(new Instruction(CPUOpCode.ADD, new Operand(CPUOperType.Register, 11), new Operand(CPUOperType.Direct, 5)));
            Console.WriteLine(new Instruction(CPUOpCode.JMP, new Operand(CPUOperType.Immediate, 3), new Operand()));

            var inst = new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Register, 5), new Operand(CPUOperType.Immediate, 0xDEAD));

            Console.WriteLine(inst.ToAssembly());
        }
    }
}
