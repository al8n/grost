// impl<K, V, S> DefaultRepeatedEntryWireFormat<Groto> for HashMap<K, V, S> {
//   type Format<KM, VM, const TAG: u32>
//     = RepeatedEntry<KM, VM, TAG>
//   where
//     KM: WireFormat<Groto> + 'static,
//     VM: WireFormat<Groto> + 'static,
//     MergedWireFormat<KM, VM>: WireFormat<Groto> + 'static;
// }
