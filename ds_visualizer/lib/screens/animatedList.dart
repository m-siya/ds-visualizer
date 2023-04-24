import 'package:flutter/material.dart';

class MyAnimatedStack extends StatelessWidget {
  const MyAnimatedStack({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SimpleAnimatedList(),
    );
  }
}

class SimpleAnimatedList extends StatefulWidget {
  const SimpleAnimatedList({super.key});

  @override
  State<SimpleAnimatedList> createState() => _SimpleAnimatedListState();
}

class _SimpleAnimatedListState extends State<SimpleAnimatedList> {
  @override
  final GlobalKey<AnimatedListState> listKey = GlobalKey<AnimatedListState>();
  List<int> _items = [1, 2, 3, 4];
  int counter = 0;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: <Widget>[
        Expanded(
          child: Container(
            height: double.infinity,
            child: AnimatedList(
              key: listKey,
              initialItemCount: _items.length,
              itemBuilder: (context, index, animation) {
                return slideIt(context, index, animation);
              },
            ),
          ),
        ),
        Container(
          child: Row(children: <Widget> [
            FloatingActionButton(onPressed: removeItem),
            FloatingActionButton(onPressed: insertItem)
          ],)
        )
      ],
    );
  }

  void removeItem() {
    if (_items.isEmpty) return;

    listKey.currentState?.removeItem(
        0, (_, animation) => slideIt(context, 0, animation),
        duration: const Duration(milliseconds: 500));

    setState(() {
      _items.removeAt(0);
    });
  }

  void insertItem() {
    setState(() {
      listKey.currentState?.insertItem(0, duration: const Duration(milliseconds: 500));
      _items = []
        ..add(counter++)
        ..addAll(_items);
    });
  }

  Widget slideIt(BuildContext context, int index, animation) {
    int item = _items[index];
    return SlideTransition(
        position: Tween<Offset>(
          begin: const Offset(-1, 0),
          end: Offset(0, 0),
        ).animate(animation),
        child: SizedBox(
            height: 128.0,
            child: Card(
                child: Center(
              child: Text('Item $item'),
            ))));
  }
}
