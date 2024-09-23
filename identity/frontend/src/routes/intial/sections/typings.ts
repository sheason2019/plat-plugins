export enum InitialSections {
  Welcome,
  Form,
  Complete,
}

export interface SectionProps {
  setSection(section: InitialSections): void;
}
