import { isDevMode } from '@angular/core';

const ApiHost = isDevMode() ? 'http://localhost:3300' : window.origin;

export { ApiHost };
