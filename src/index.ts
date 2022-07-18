import MathFunctions from './Math';
import DateFunctions from './Date/index';
import ToolFunctions from './Tool';
import ArrayFunctions from './Array';

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
