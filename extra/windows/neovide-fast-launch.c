#define UNICODE
#define _UNICODE
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <strsafe.h>
#include <wchar.h>

int WINAPI wWinMain(HINSTANCE instance, HINSTANCE previous, PWSTR arguments, int show) {
    (void)instance;
    (void)previous;
    (void)show;
    WCHAR executable[32768];
    WCHAR command[32768];
    DWORD length = GetModuleFileNameW(NULL, executable, ARRAYSIZE(executable));
    if (!length || length >= ARRAYSIZE(executable)) return 1;
    WCHAR *filename = wcsrchr(executable, L'\\');
    if (!filename) return 1;
    ++filename;
    if (FAILED(StringCchCopyW(filename, ARRAYSIZE(executable) - (filename - executable), L"neovide.exe"))) return 1;
    if (FAILED(StringCchPrintfW(command, ARRAYSIZE(command), L"\"%s\" --opengl %s", executable, arguments))) return 1;

    STARTUPINFOW startup = {0};
    startup.cb = sizeof(startup);
    startup.dwFlags = STARTF_USESHOWWINDOW;
    startup.wShowWindow = SW_HIDE;
    PROCESS_INFORMATION process = {0};
    if (!CreateProcessW(executable, command, NULL, NULL, FALSE, 0, NULL, NULL, &startup, &process)) {
        DWORD error = GetLastError();
        WCHAR message[128];
        StringCchPrintfW(message, ARRAYSIZE(message), L"Could not start experimental Neovide (Windows error %lu).", error);
        MessageBoxW(NULL, message, L"Neovide test launcher", MB_OK | MB_ICONERROR);
        return 1;
    }
    CloseHandle(process.hThread);
    CloseHandle(process.hProcess);
    return 0;
}
