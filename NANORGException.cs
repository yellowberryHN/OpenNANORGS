using System;

namespace OpenNANORGS;

public class NANORGException : Exception
{
    public NANORGException()
    {
    }

    public NANORGException(string message): base(message)
    {
    }

    public NANORGException(string message, Exception inner) : base(message, inner)
    {
    }
}