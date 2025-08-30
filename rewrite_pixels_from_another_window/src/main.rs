use windows::{ core::*,
    Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::UI::WindowsAndMessaging::*
};

fn draw(hwnd: HWND) {
    unsafe {
        
        let color = COLORREF(0x0000FF);
        let mut window_size = RECT::default();

        loop {
            if let Ok(_) = GetClientRect(hwnd, &mut window_size) {
                let rect = RECT {
                    left: 0,
                    top: 0,
                    right: window_size.right,
                    bottom: window_size.bottom,
                };

                let hdc = GetDC(Some(hwnd));
                let brush = CreateSolidBrush(color);

                FillRect(hdc, &rect, brush);

                if !DeleteObject(HGDIOBJ::from(brush)).as_bool() {
                    println!("Resource leak prevented");
                    break;
                }
                ReleaseDC(Some(hwnd), hdc);
            }
        }
    }
}

fn main() {
    unsafe {
        // choose window you want to rewrite 
        if let Ok(hwnd) = FindWindowA(None, PCSTR(b"Tlauncher 2.9307\0".as_ptr())) {
            draw(hwnd);
        }
    }
}

/*
❓Хочешь нарисовать поверх чужого окна, как оверлей?
Если ты хочешь делать "оверлей" — например, рисовать поверх окна игры, браузера и т.п., то:

Обычно создают своё прозрачное окно поверх нужного окна (через WS_EX_LAYERED, WS_EX_TRANSPARENT).

Или рисуют напрямую через GetDC(hwnd).

Могу помочь создать такой оверлей или объяснить, как рисовать "правильно" и безопасно.
*/


/*
❌ Ошибка:
// Нельзя так делать — это чужое окно!
let hdc = BeginPaint(hwnd, &mut ps);  // ❌

✅ Правильно:
let hdc = GetDC(Some(hwnd));          // ✅ Получили DC чужого окна
FillRect(hdc, &rect, brush);          // ✅ Нарисовали
ReleaseDC(Some(hwnd), hdc);           // ✅ Освободили DC

🧠 Почему это важно?
Окна в Windows обрабатываются своими процессами. 
Когда вы вызываете BeginPaint, вы говорите: "Я — владелец окна, и сейчас обрабатываю WM_PAINT". 
Но если окно принадлежит другому процессу — вы не имеете права так делать. 
Это может нарушить внутреннюю логику работы окна и системы.
*/

/*
🔍 Как работает WS_EX_LAYERED:
Это специальный флаг расширенного стиля окна, который позволяет:

рисовать с альфа-прозрачностью (включая полупрозрачные пиксели)

делать окно полностью или частично прозрачным

отображать окно поверх всех остальных окон, но при этом пропускать мышь (WS_EX_TRANSPARENT)
*/ 