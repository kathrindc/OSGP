import { isDevMode } from '@angular/core';

const ApiHost = isDevMode() ? 'http://localhost:8000' : window.origin;

export { ApiHost };
