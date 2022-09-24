package doubleylinkedlist

import (
	"testing"
)
func TestLinkedListConstructor(t *testing.T) {
    list := Constructor()
    if list.Head != nil {
        t.Fail()
    }
}

func TestAddToHead(t *testing.T) {
    list := Constructor()
    val := 1
    list.AddAtHead(val)
    if list.Head.Val != val {
        t.Fail()
    }
}

func TestAddToHeadMultiple(t *testing.T) {
    list := Constructor()
    list.AddAtHead(2)
    list.AddAtHead(1)
    if list.Head.Next.Prev != list.Head {
        t.Fail()
    }
}

func TestGetHead(t *testing.T) {
    list := Constructor()
    val := 1
    list.AddAtHead(val)
    if list.Get(0) != val {
        t.Fail()
    }
}

func TestGetSecondElement(t *testing.T) {
    list := Constructor()
    val := 1
    list.AddAtHead(val)
    list.AddAtHead(0)
    if list.Get(1) != val {
        t.Fail()
    }
}


func TestGetOutOfBounds (t *testing.T) {
    list := Constructor()
    list.AddAtHead(4)
    if list.Get(1) != -1 {
        t.Fail()
    }
}

func TestGetNext (t *testing.T) {
    list := Constructor()
    list.AddAtHead(2)
    list.AddAtHead(1)
    list.AddAtIndex(2,1)
    if list.Get(1) != 2 {
        t.Fail()
    }
}

func TestAddToTail(t *testing.T) {
    list := Constructor()
    val := 1
    list.AddAtTail(0)
    list.AddAtTail(val)
    if list.Get(1) != val {
        t.Fail()
    }
}

func TestAddToFirstIndex(t *testing.T) {
    list := Constructor()
    val := 1
    list.AddAtIndex(0, val)
    if list.Get(0) != val {
        t.Fail()
    }
}

func TestAddToSecondIndexEnd(t *testing.T) {
    list := Constructor()
    val := 1
    list.AddAtHead(0)
    list.AddAtIndex(1, val)
    if list.Get(1) != val {
        t.Fail()
    }
}

func TestAddToSecondIndexMiddle(t *testing.T) {
    list := Constructor()
    val := 1
    list.AddAtHead(0)
    list.AddAtTail(2)
    list.AddAtIndex(1, val)
    if list.Get(1) != val && list.Get(2) != 2{
        t.Fail()
    }
}


func TestAddHeadTwice(t *testing.T) {
    list := Constructor()
    list.AddAtIndex(0,10)
    list.AddAtIndex(0, 20)
    list.AddAtIndex(1, 30)
    if list.Get(0) != 20 {
        t.Fail()
    }
}


func TestDeleteFromEmptyList(t *testing.T) {
    list := Constructor()
    list.DeleteAtIndex(0)
    if list.Head != nil {
        t.Fail()
    }
}

func TestDeleteHeadOfList(t *testing.T) {
    list := Constructor()
    list.AddAtHead(0)
    list.AddAtIndex(1, 1)
    list.DeleteAtIndex(0)
    if list.Get(0) != 1 {
        t.Fail()
    }
}
func TestDeleteSecondElementOfList(t *testing.T) {
    list := Constructor()
    list.AddAtHead(0)
    list.AddAtIndex(1, 1)
    list.AddAtTail(2)
    list.DeleteAtIndex(1)
    if list.Get(0) != 0 && list.Get(1) == 2 {
        t.Fail()
    }
}
func TestDeleteHead(t *testing.T) {
    list := Constructor()
    list.AddAtHead(1)
    list.DeleteAtIndex(0)
    if list.Head != nil {
        t.Fail()
    }
}

func TestCombination(t *testing.T) {
    list := Constructor()
    list.AddAtHead(2)
    list.DeleteAtIndex(1)
    list.AddAtHead(2)
    list.AddAtHead(7)
    list.AddAtHead(3)
    list.AddAtHead(2)
    list.AddAtHead(5)
    list.AddAtTail(5)
    list.Get(5)
    list.DeleteAtIndex(6)
    list.DeleteAtIndex(4)
    if list.Head != nil {
        t.Fail()
    }
}






