using System;

namespace NANORG_CPU
{
    class Program
    {
        static void Main(string[] args)
        {

            // make sure to account for relative positioning on jumps and calls (cringe)

            /*
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Register, 5), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Register, 5), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Direct, 5), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 5, 255), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 5, 9, true), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 9, 12), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Direct, 12), new Operand(CPUOperType.Immediate, 0xDEAD)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Register, 3), new Operand(CPUOperType.Register, 4)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 4, 7), new Operand(CPUOperType.RegisterIndexed, 4, 7)));
            Console.WriteLine(new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 4, 7, true), new Operand(CPUOperType.RegisterIndexed, 4, 7, true)));
            Console.WriteLine(new Instruction(CPUOpCode.SUB, new Operand(CPUOperType.Register, 6), new Operand(CPUOperType.RegisterIndexed, 4, 7, true)));
            Console.WriteLine(new Instruction(CPUOpCode.ADD, new Operand(CPUOperType.Register, 11), new Operand(CPUOperType.Direct, 5)));
            Console.WriteLine(new Instruction(CPUOpCode.JMP, new Operand(CPUOperType.Immediate, 6), new Operand(), 9));
            */

            /*
            Console.WriteLine(new Instruction(CPUOpCode.RAND, new Operand(CPUOperType.Register, 0), new Operand(CPUOperType.Immediate, 4), 0));
            Console.WriteLine(new Instruction(CPUOpCode.TRAVEL, new Operand(CPUOperType.Register, 0), null, 3));
            Console.WriteLine(new Instruction(CPUOpCode.EAT, null, null, 6));
            Console.WriteLine(new Instruction(CPUOpCode.SENSE, new Operand(CPUOperType.Register, 0), null, 9));
            Console.WriteLine(new Instruction(CPUOpCode.CMP, new Operand(CPUOperType.Register, 0), new Operand(CPUOperType.Immediate, 0xFFFF), 12));
            Console.WriteLine(new Instruction(CPUOpCode.JNE, new Operand(CPUOperType.Immediate, 0),null, 15));
            Console.WriteLine(new Instruction(CPUOpCode.RELEASE, new Operand(CPUOperType.Immediate, 10000), null));
            Console.WriteLine(new Instruction(CPUOpCode.JMP, new Operand(CPUOperType.Immediate, 0), null, 21));
            */

            /*
            var inst = new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.Register, 5), new Operand(CPUOperType.Immediate, 0xDEAD));
            //inst = new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 5, 9), new Operand(CPUOperType.Immediate, 0xDEAD));
            //inst = new Instruction(CPUOpCode.MOV, new Operand(CPUOperType.RegisterIndexed, 4, 7), new Operand(CPUOperType.RegisterIndexed, 4, 7));
            inst = new Instruction(CPUOpCode.TRAVEL, new Operand(CPUOperType.Register, 0), null);
            inst = new Instruction(CPUOpCode.RAND, new Operand(CPUOperType.Register, 0),
                new Operand(CPUOperType.Immediate, 4));
            
            Console.WriteLine(inst.ToAssembly());
            */

            var parser = new Parser("testbot.asm");
            var insts = parser.instructionList;

        }
    }
}
