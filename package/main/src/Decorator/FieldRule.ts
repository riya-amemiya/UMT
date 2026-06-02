export interface FieldRule {
  validate: (value: unknown) => boolean;
  message: string;
}
