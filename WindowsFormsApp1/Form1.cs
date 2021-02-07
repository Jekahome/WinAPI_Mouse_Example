using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace WindowsFormsApp1
{
    public partial class Form1 : Form
    {
       

        [DllImport("user32.dll", CharSet = CharSet.Auto, CallingConvention = CallingConvention.StdCall)]
        public static extern void mouse_event(long dwFlags, long dx, long dy, long cButtons, long dwExtraInfo);
        private const int MOUSEEVENTF_LEFTDOWN = 0x02;
        private const int MOUSEEVENTF_LEFTUP = 0x04;
        private const int MOUSEEVENTF_ABSOLUTE = 0x8000;


        System.Threading.Timer _timer;
        public Form1()
        {
            InitializeComponent();
            Random r = new Random();
            // По нажатию активации поиска координат
            // Камера на raspebery отслеживает шаблонный обьект ближний к курсору и отправляет в это приложение на ПК координаты обьекта
            // Курсор мыши перемещается в нужное место координат заметая след
            _timer = new System.Threading.Timer(A, null, 0, r.Next(4000, 10000));
        }


        public void DoMouseClick()
        {
            int X = Cursor.Position.X;
            int Y = Cursor.Position.Y;
           // mouse_event(MOUSEEVENTF_LEFTDOWN, X, Y, 0, 0);
           // mouse_event(MOUSEEVENTF_LEFTUP, X, Y, 0, 0);

            mouse_event(MOUSEEVENTF_ABSOLUTE, X, Y, 0, 0);

        }



        void A(object o)
        {
            Random r = new Random();
            Cursor.Position = new Point(100, 500);
            DoMouseClick();
            
            Thread.Sleep(r.Next(1000, 3000));
            Cursor.Position = new Point(100, 1000);
            DoMouseClick();
           /*
            Thread.Sleep(r.Next(1000, 3000));
            Cursor.Position = new Point(1000, 1000);
            DoMouseClick();
            
            Thread.Sleep(r.Next(1000, 3000));
            Cursor.Position = new Point(1000, 5000);
            DoMouseClick();*/

        }

    }

}


