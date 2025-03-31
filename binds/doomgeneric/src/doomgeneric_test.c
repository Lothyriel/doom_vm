#include "doomgeneric.h"
#include "doomkeys.h"

void DG_Init() {}

void DG_DrawFrame() { DG_ScreenBuffer; }

void DG_SleepMs(uint32_t ms) {}

uint32_t DG_GetTicksMs() {}

int DG_GetKey(int *pressed, unsigned char *doomKey) {
  // need to set the pressed if the key is pressed
  // need to return if a key was retrived
  //
}

void DG_SetWindowTitle(const char *title) {}
