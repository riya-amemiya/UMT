import { isBrowser } from './isBrowser';
import { isNode } from './isNode';

export const isNodeWebkit = isBrowser && isNode;
