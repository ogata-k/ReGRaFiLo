# .regrfのファイル形式
regrf形式のファイルは先にみたように属性無しのXML風の形式で記述します。
XML同様タグを用いて宣言していきますが、XMLと違いいくつかの制限がかかっています。
その制限は
   1. 属性を利用できない
   1. タグ名はハイフンが文字列の終始にならない文字列で、半角英文字の小文字とハイフンのみからなる
   1. <hoge/>形式の中身を持たないタグは利用できない
   1. 値を持たない場合は<hoge></hoge>としなければいけない
   
となっています。

.regrfでは最初にレイアウト構造を定義し、次にグラフを定義していく形で記述していきます。
regrf形式の全体の構造は次のように表現されます。

```xml:example.regrf
<regrf>
    <meta>
        <charset>utf8</charset>
        <img-size>
            <height>height</height>
            <width>width</width>
        </img-size>
    </meta>
    <layout>
        <enumerate>
            <colors>
                <color>color</color>
            </colors>
        </enumerate>
        <structure>
            <labels>
                <label>label structure</label>
            </labels>
            <graphs>
                <graph>inner graph structure</graph>
            </graphs>
            <nodes>
                <node>node structure</node>
            </nodes>
            <edges>
                <edge>edge structure</edge>
            </edges>
        </structure>
    </layout>
    <outer-graph>
        <nodes>
            <node>node body</node>
        </nodes>
        <inner-graphs>
            <inner-graph>inner graph body</inner-graph>
        </inner-graphs>
        <edges>
            <edge>edge body</edge>
        </edges>
    </outer-graph>
</regrf>
```
このように.regrfは属性を用いないXML風の形式で記述されます。
中身を見ていくと、regrfタグの下にはmeta、layout、outer-graphタグが定義されています。これらは必須です。<br/>
では、さっそく何を宣言しているのか順番に見ていきます。
## meta
まず最初はmetaタグです。<br/>
metaタグは画像ファイルの形式を設定するために使用するタグです。文字集合(charset)と画像としてのサイズ(img-size)が指定できます。<br/>
charsetタグで使用できるのはutf8だけですが、ゆくゆく他の形式に対応するかもしれません。<br/>
img-sizeタグではheight、widthタグを使用してサイズを定義します。使用可能な値はともに非負の整数値となります。<br/>
もしこれらを指定しなかった場合、文字集合はutf8で画像サイズが自動で設定されます。
ちなみに画像サイズにおいては指定していたとしても、変換時に値が指定されていればその値で上書きされます。

## layout
次にlayoutタグについて見ていきます。<br/>
layoutタグではレイアウトで用いる定数を列挙するenumerateタグと、グラフをきれいに見せるためのstructureタグがあります。<br/>
enumerateタグではcolorsタグを宣言できます。基本的な色はここで宣言しなくてもサポートするつもりです。
しかし、バージョンによって色が微妙に変わってくる恐れがあるので、ここで宣言しておいた方がよいと思います。<br/>
colorsタグの中では描画色を表すcolorタグを宣言できます。colorタグは一意的な名前を表すidタグと色を指定するタグからなります。
色を指定するタグはrgbaやrgb、hsl、hsv、cmykタグが使用できます。
これらのタグではrgb系統の```(244,44,12,255)```のように0から255を使った指定やその他の系統の```(100, 23, 9)```のように0から100を使った指定ができます。
その上、```#A011220B```のように#から始まるHex形式の指定もできます。<br/>
structureタグではlabels、graphs、nodes、edgesタグが宣言できます。
それぞれ文字フォント、（内部）グラフ、頂点、辺のレイアウト構造を定義します。<br/>
どのタグの中で宣言されるタブでも一意的な名前を表すnameタグと構造を宣言するtypeタグは必須となっています。
さらにtypeタグはユーザー宣言型かデフォルトで用意されている型かを区別するためのclassタグが必須で、デフォルト設定を上書きするためのstyleタグが任意で指定できます。
それぞれの設定項目の説明は省略します。

## outer-graph
最後はouter-graphタグについてです。<br/>
先ほどgraphsタグが出てきました。こちらのタグは頂点としても利用できるグラフ、つまり内部グラフについての構造を定義するためのタグでした。
それとは違いこのouter-graphタグは描画しようと考えているグラフの全体を表現するグラフとなります。つまりフォルダ構成で言うルートフォルダと同じ扱いになります。<br/>
このouter-graphタグで指定できるタグは頂点と同様に扱うことを示すidタグ（辺の場合は辺の真ん中を指す）や[layout](#layout)で説明したのと同様の意味のtypeタグ、グラフの本体であるbodyタグです。
ぶっちゃけレイアウトや構造を考えずにグラフとして扱うことを考えるだけならbodyタグ直下とidタグだけで十分です。
