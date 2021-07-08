using System;
using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;

namespace App {

class Program {
    [DllImport("__Internal")]
    public static extern void HelloWorldRust();

    static void Main(string[] args) {
        Console.WriteLine("Hello World!");
        HelloWorldRust();

        Hello.DoAThing();
        Hello.PrintString("this is mono string");
        var r_str = Hello.GetString();
        Console.WriteLine($"rust string: {r_str}");

        var person = new Person("c#Person");
        person.Greet();
    }

    static void CallFromRust() {
        Console.WriteLine("c# called from rust");
    }
    
    static void PrintNumber(UInt32 num, UInt32 num2) {
        Console.WriteLine($"c# num: {num}, {num2}");
    }
}

class Person {
    private string _name;

    public Person() {
        _name = "DefaultName";
    }

    public Person(string name) {
        _name = name;
    }

    public void SetName(string name) {
        _name = name;
    }

    public void Greet() {
        Console.WriteLine($"Hello, {_name}");
    }
}

static class Hello {
    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void DoAThing();

    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern void PrintString(string s);

    [MethodImplAttribute(MethodImplOptions.InternalCall)]
    public static extern string GetString();
}

}
