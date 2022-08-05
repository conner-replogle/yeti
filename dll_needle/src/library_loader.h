#include "Windows.h";
typedef HMODULE(__stdcall* pLoadLibraryA)(LPCSTR);
typedef FARPROC(__stdcall* pGetProcAddress)(HMODULE, LPCSTR);

typedef INT(__stdcall* dllmain)(HMODULE, DWORD, LPVOID);
struct Loader_Data
{
	LPVOID ImageBase;

	PIMAGE_NT_HEADERS NtHeaders;
	PIMAGE_BASE_RELOCATION BaseReloc;
	PIMAGE_IMPORT_DESCRIPTOR ImportDirectory;

	pLoadLibraryA fnLoadLibraryA;
	pGetProcAddress fnGetProcAddress;
    LPVOID extraData;
};

void write_loader_data(struct Loader_Data *LoaderParams);
// HANDLE call_lib(HANDLE hProcess,PVOID addy);
DWORD __stdcall library_loader(LPVOID Memory);
DWORD __stdcall stub();