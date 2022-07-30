import * as MathFunctions from './Math';
import * as DateFunctions from './Date/index';
import * as ToolFunctions from './Tool';
import * as ArrayFunctions from './Array';

const UMT: {
    Array: typeof ArrayFunctions;
    Math: typeof MathFunctions;
    Date: typeof DateFunctions;
    Tool: typeof ToolFunctions;
} = {
    Array: ArrayFunctions,
    Math: MathFunctions,
    Date: DateFunctions,
    Tool: ToolFunctions,
};
export default UMT;
