(function() {var implementors = {};
implementors["pushgen"] = [{"text":"impl&lt;First:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, Second:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.Chain.html\" title=\"struct pushgen::structs::Chain\">Chain</a>&lt;First, Second&gt;","synthetic":false,"types":["pushgen::structs::chain::Chain"]},{"text":"impl&lt;Src:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.Cloned.html\" title=\"struct pushgen::structs::Cloned\">Cloned</a>&lt;Src&gt;","synthetic":false,"types":["pushgen::structs::cloned::Cloned"]},{"text":"impl&lt;Src:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.Copied.html\" title=\"struct pushgen::structs::Copied\">Copied</a>&lt;Src&gt;","synthetic":false,"types":["pushgen::structs::copied::Copied"]},{"text":"impl&lt;Src:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.Dedup.html\" title=\"struct pushgen::structs::Dedup\">Dedup</a>&lt;Src&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Src: <a class=\"trait\" href=\"pushgen/trait.Generator.html\" title=\"trait pushgen::Generator\">Generator</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Src::<a class=\"type\" href=\"pushgen/trait.Generator.html#associatedtype.Output\" title=\"type pushgen::Generator::Output\">Output</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Src::<a class=\"type\" href=\"pushgen/trait.Generator.html#associatedtype.Output\" title=\"type pushgen::Generator::Output\">Output</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["pushgen::structs::dedup::Dedup"]},{"text":"impl&lt;Gen:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, Pred:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.Filter.html\" title=\"struct pushgen::structs::Filter\">Filter</a>&lt;Gen, Pred&gt;","synthetic":false,"types":["pushgen::structs::filter::Filter"]},{"text":"impl&lt;Gen:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, Func:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.FilterMap.html\" title=\"struct pushgen::structs::FilterMap\">FilterMap</a>&lt;Gen, Func&gt;","synthetic":false,"types":["pushgen::structs::filter_map::FilterMap"]},{"text":"impl&lt;Src&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.Flatten.html\" title=\"struct pushgen::structs::Flatten\">Flatten</a>&lt;Src&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Src: <a class=\"trait\" href=\"pushgen/trait.Generator.html\" title=\"trait pushgen::Generator\">Generator</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Src::<a class=\"type\" href=\"pushgen/trait.Generator.html#associatedtype.Output\" title=\"type pushgen::Generator::Output\">Output</a>: <a class=\"trait\" href=\"pushgen/trait.IntoGenerator.html\" title=\"trait pushgen::IntoGenerator\">IntoGenerator</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Src::<a class=\"type\" href=\"pushgen/trait.Generator.html#associatedtype.Output\" title=\"type pushgen::Generator::Output\">Output</a> as <a class=\"trait\" href=\"pushgen/trait.IntoGenerator.html\" title=\"trait pushgen::IntoGenerator\">IntoGenerator</a>&gt;::<a class=\"type\" href=\"pushgen/trait.IntoGenerator.html#associatedtype.IntoGen\" title=\"type pushgen::IntoGenerator::IntoGen\">IntoGen</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["pushgen::structs::flatten::Flatten"]},{"text":"impl&lt;F:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.FromFn.html\" title=\"struct pushgen::structs::FromFn\">FromFn</a>&lt;F&gt;","synthetic":false,"types":["pushgen::structs::from_fn::FromFn"]},{"text":"impl&lt;I:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.FromIter.html\" title=\"struct pushgen::structs::FromIter\">FromIter</a>&lt;I&gt;","synthetic":false,"types":["pushgen::structs::from_iter::FromIter"]},{"text":"impl&lt;Src:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.IteratorAdaptor.html\" title=\"struct pushgen::structs::IteratorAdaptor\">IteratorAdaptor</a>&lt;Src&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Src: <a class=\"trait\" href=\"pushgen/trait.Generator.html\" title=\"trait pushgen::Generator\">Generator</a>,&nbsp;</span>","synthetic":false,"types":["pushgen::structs::iterator::IteratorAdaptor"]},{"text":"impl&lt;Gen:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, Func:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.Map.html\" title=\"struct pushgen::structs::Map\">Map</a>&lt;Gen, Func&gt;","synthetic":false,"types":["pushgen::structs::map::Map"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.OptionGen.html\" title=\"struct pushgen::structs::OptionGen\">OptionGen</a>&lt;T&gt;","synthetic":false,"types":["pushgen::structs::option::OptionGen"]},{"text":"impl&lt;Gen:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.Skip.html\" title=\"struct pushgen::structs::Skip\">Skip</a>&lt;Gen&gt;","synthetic":false,"types":["pushgen::structs::skip::Skip"]},{"text":"impl&lt;Src:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, P:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.SkipWhile.html\" title=\"struct pushgen::structs::SkipWhile\">SkipWhile</a>&lt;Src, P&gt;","synthetic":false,"types":["pushgen::structs::skip::SkipWhile"]},{"text":"impl&lt;Src:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.Take.html\" title=\"struct pushgen::structs::Take\">Take</a>&lt;Src&gt;","synthetic":false,"types":["pushgen::structs::take::Take"]},{"text":"impl&lt;Src:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, P:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.TakeWhile.html\" title=\"struct pushgen::structs::TakeWhile\">TakeWhile</a>&lt;Src, P&gt;","synthetic":false,"types":["pushgen::structs::take::TakeWhile"]},{"text":"impl&lt;Left:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, Right:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/structs/struct.Zip.html\" title=\"struct pushgen::structs::Zip\">Zip</a>&lt;Left, Right&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Left: <a class=\"trait\" href=\"pushgen/trait.Generator.html\" title=\"trait pushgen::Generator\">Generator</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Left::<a class=\"type\" href=\"pushgen/trait.Generator.html#associatedtype.Output\" title=\"type pushgen::Generator::Output\">Output</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,&nbsp;</span>","synthetic":false,"types":["pushgen::structs::zip::Zip"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"pushgen/enum.ValueResult.html\" title=\"enum pushgen::ValueResult\">ValueResult</a>","synthetic":false,"types":["pushgen::ValueResult"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"enum\" href=\"pushgen/enum.GeneratorResult.html\" title=\"enum pushgen::GeneratorResult\">GeneratorResult</a>","synthetic":false,"types":["pushgen::GeneratorResult"]},{"text":"impl&lt;'a, T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"pushgen/struct.SliceGenerator.html\" title=\"struct pushgen::SliceGenerator\">SliceGenerator</a>&lt;'a, T&gt;","synthetic":false,"types":["pushgen::SliceGenerator"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()