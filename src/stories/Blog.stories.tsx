import { LayeredVsNonLayered } from './LayeredVsNonLayered';
import { TidyExample } from './TidyExample';

export default {
  title: 'Blog',
};

export function Layered() {
  return <LayeredVsNonLayered />;
}

export function InteractiveTidy() {
  return <TidyExample />;
}
