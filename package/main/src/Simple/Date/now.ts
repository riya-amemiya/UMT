import { now } from "@/Date/now";
import { isNumber } from "@/Math/isNumber";
import { hoursType, hoursTypeInt } from "@/types/clockType";

export const nowSimple = (timeDifference: hoursTypeInt | hoursType = 9) => {
<<<<<<< HEAD
  if (typeof timeDifference === "number") {
    return now(timeDifference);
  } else if (isNumber(timeDifference)) {
    return now(Number(timeDifference) as hoursTypeInt);
  } else {
    return now();
  }
=======
	if (typeof timeDifference === "number") {
		return now(timeDifference);
	} else if (isNumber(timeDifference)) {
		return now(Number(timeDifference) as hoursTypeInt);
	} else {
		return now();
	}
>>>>>>> origin/main
};
