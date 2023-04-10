import { isDevMode } from '@angular/core';

const ApiDomain = isDevMode() ? 'localhost:8000' : location.host;
const ApiHost = `${isDevMode() ? 'http' : 'https'}://${ApiDomain}`;

export { ApiHost, ApiDomain };
