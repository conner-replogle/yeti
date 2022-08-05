#include "library_loader.h"




void write_loader_data(struct Loader_Data *LoaderParams){

	LoaderParams->fnLoadLibraryA = LoadLibraryA;
	LoaderParams->fnGetProcAddress = GetProcAddress;
	// Write the loader information to target process
}
// HANDLE call_lib(HANDLE hProcess,PVOID addy){
//     //std::cout << a << std::endl;
//     // Write the loader code to target process
//     WriteProcessMemory(hProcess, (PVOID)((struct Loader_Data*)addy+1), library_loader,//(PVOID)((loader_data*)addy + 1)
// 		(DWORD)stub - (DWORD)library_loader, NULL);
//     return CreateRemoteThread(hProcess,NULL,0,(LPTHREAD_START_ROUTINE)((struct Loader_Data*)addy+1),(LPVOID)addy,0,NULL);//(LPTHREAD_START_ROUTINE)((loader_data*)addy + 1)
// }

DWORD __stdcall  library_loader(LPVOID Memory) {
    struct Loader_Data* LoaderParams = (struct Loader_Data*)Memory;
	PIMAGE_BASE_RELOCATION pIBR = LoaderParams->BaseReloc;

	DWORD delta = (DWORD)((LPBYTE)LoaderParams->ImageBase - LoaderParams->NtHeaders->OptionalHeader.ImageBase); // Calculate the delta
    
	while (pIBR->VirtualAddress)
	{
		if (pIBR->SizeOfBlock >= sizeof(IMAGE_BASE_RELOCATION))
		{
			int count = (pIBR->SizeOfBlock - sizeof(IMAGE_BASE_RELOCATION)) / sizeof(WORD);
			PWORD list = (PWORD)(pIBR + 1);

			for (int i = 0; i < count; i++)
			{
				if (list[i])
				{
					PDWORD ptr = (PDWORD)((LPBYTE)LoaderParams->ImageBase + (pIBR->VirtualAddress + (list[i] & 0xFFF)));
					*ptr += delta;
				}
			}
		}

		pIBR = (PIMAGE_BASE_RELOCATION)((LPBYTE)pIBR + pIBR->SizeOfBlock);
	}

	PIMAGE_IMPORT_DESCRIPTOR pIID = LoaderParams->ImportDirectory;
    //ALLGOOD

	// Resolve DLL imports
	while (pIID->Characteristics)
	{
		PIMAGE_THUNK_DATA OrigFirstThunk = (PIMAGE_THUNK_DATA)((LPBYTE)LoaderParams->ImageBase + pIID->OriginalFirstThunk);
		PIMAGE_THUNK_DATA FirstThunk = (PIMAGE_THUNK_DATA)((LPBYTE)LoaderParams->ImageBase + pIID->FirstThunk);

		HMODULE hModule = LoaderParams->fnLoadLibraryA((LPCSTR)LoaderParams->ImageBase + pIID->Name);

		if (!hModule)
			return FALSE;

		while (OrigFirstThunk->u1.AddressOfData)
		{
			if (OrigFirstThunk->u1.Ordinal & IMAGE_ORDINAL_FLAG)
			{
				// Import by ordinal
				DWORD Function = (DWORD)LoaderParams->fnGetProcAddress(hModule,
					(LPCSTR)(OrigFirstThunk->u1.Ordinal & 0xFFFF));

				if (!Function)
					return FALSE;

				FirstThunk->u1.Function = Function;
			}
			else
			{
				// Import by name
				PIMAGE_IMPORT_BY_NAME pIBN = (PIMAGE_IMPORT_BY_NAME)((LPBYTE)LoaderParams->ImageBase + OrigFirstThunk->u1.AddressOfData);
				DWORD Function = (DWORD)LoaderParams->fnGetProcAddress(hModule, (LPCSTR)pIBN->Name);
				if (!Function)
					return FALSE;

				FirstThunk->u1.Function = Function;
			}
			OrigFirstThunk++;
			FirstThunk++;
		}
		pIID++;
	}
    DWORD addy_entry = LoaderParams->NtHeaders->OptionalHeader.AddressOfEntryPoint;
    LPVOID image_base = LoaderParams->ImageBase;
    //SecureZeroMemory(LoaderParams->ImageBase,4096);
	if (addy_entry)
	{
		dllmain EntryPoint = (dllmain)((LPBYTE)image_base + addy_entry);

		return EntryPoint((HMODULE)image_base, DLL_PROCESS_ATTACH, LoaderParams->extraData); // Call the entry point
	}
	return TRUE;
}
DWORD __stdcall stub(){
    return 0;
}