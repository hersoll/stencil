<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from '../ServerMessage.svelte';
  import { type CourseEntryRaw, type Entry } from '../types';

  let {
    course_ids = $bindable(),
    serverMessage,
    draggedEntry,
    dropPriority = $bindable(),
    parentDraggedOver,
    entry = $bindable()
  }: {
    course_ids: number[];
    serverMessage: ServerMessage;
    draggedEntry: Entry | null;
    dropPriority: boolean;
    parentDraggedOver: boolean;
    entry: Entry;
  } = $props();
  let draggedOver = $state(false);
  let dragDepth = $state(0);
  let draggedCourse = $state<CourseEntryRaw | null>(null);
  let draggedIndex = $state(-1);
  let courses = $state<CourseEntryRaw[]>([]);

  function inCourses(course: CourseEntryRaw): boolean {
    return courses.find(c => c.id == course.id) !== undefined;
  }

  function addCourse(course: CourseEntryRaw) {
    courses.push(course);
    course_ids.push(course.id);
  }

  function removeCourse(course: CourseEntryRaw) {
    courses = courses.filter(c => c.id !== course.id);
    course_ids = course_ids.filter(id => id !== course.id);
  }

  // The topic area is entered while dragging
  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    dragDepth++;
    dropPriority = true;
    draggedOver = true;
  }

  function handleDragLeave() {
    dragDepth--;
    if (dragDepth == 0) {
      dropPriority = false;
      draggedOver = false;
    }
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    // Only allow non-empty topics
    if (draggedEntry?.kind === 'course' && draggedEntry?.id >= 0) {
      if (!inCourses(draggedEntry)) {
        addCourse(draggedEntry);
      }
    } else {
      dropPriority = false;
    }
    dragDepth--;
    draggedOver = false;
  }

  function handleCourseDragStart(course: CourseEntryRaw, index: number) {
    draggedCourse = course;
    draggedIndex = index;
  }

  function handleCourseDragOver(e: DragEvent, targetIndex: number) {
    e.preventDefault();

    if (draggedIndex === -1 || draggedIndex === targetIndex) return;

    const newOrder = [...courses];
    const [removed_entry] = newOrder.splice(draggedIndex, 1);
    newOrder.splice(targetIndex, 0, removed_entry);
    courses = newOrder;
    course_ids = courses.map(c => c.id);

    draggedIndex = targetIndex;
  }

  let courseWillBeRemoved = $state(false);
  function handleCourseDrag() {
    if (parentDraggedOver) {
      courseWillBeRemoved = false;
    } else if (!parentDraggedOver) {
      courseWillBeRemoved = true;
    }
  }

  function handleCourseDragEnd(course: CourseEntryRaw) {
    if (courseWillBeRemoved) {
      removeCourse(course);
    }
    courseWillBeRemoved = false;
    draggedCourse = null;
    draggedIndex = -1;
  }
  async function fetchCourse() {
    let res = await fetch(
      `${API_URL}/edit/course/from_${entry.kind}/${entry.id}`
    );
    if (res.ok) {
      courses = await res.json();
    } else {
      serverMessage.show(res);
    }
  }

  $effect(() => {
    if (course_ids || courses.length == 0) fetchCourse();
  });
</script>

<div
  role="status"
  class="courses-container"
  class:parent-dragged={parentDraggedOver}
  class:available={draggedEntry?.kind === 'course'}
  class:droppable={draggedEntry?.kind === 'course' && draggedOver}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  <h3 class="course-header">Courses</h3>
  {#each courses as course, i (course.id)}
    <button
      class="course-entry no-select"
      class:parent-dragged={parentDraggedOver}
      class:available={draggedEntry?.kind === 'course'}
      id={course.name}
      draggable="true"
      in:fly={{ y: 40, duration: 400, delay: 20 * i }}
      ondrag={handleCourseDrag}
      ondragend={() => handleCourseDragEnd(course)}
      ondragstart={() => handleCourseDragStart(course, i)}
      ondragover={e => handleCourseDragOver(e, i)}
    >
      {course.desc.sv}
    </button>
  {/each}
</div>

<style>
  .courses-container {
    grid-row: 1 / 3;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    width: 100%;
    height: var(--height);
    padding: 1rem;
    overflow-y: auto;

    background-color: var(--bg-light);
    border-radius: 1rem;
    box-shadow: var(--shadow-elevation-low);

    text-align: center;

    &.parent-dragged {
      background-color: var(--bg);
    }

    &.available {
      background-color: var(--bg-light);
    }

    &.droppable {
      box-shadow: var(--shadow-elevation-high);
    }
  }

  .course-header {
    margin-bottom: 1rem;
  }

  .course-entry {
    border: 2px solid var(--bg-dark);
    box-shadow: var(--shadow-elevation-low);
    width: 100%;
    &:hover {
      border-color: var(--primary);
    }
    &.parent-dragged {
      background-color: var(--bg);
    }
    &.available {
      background-color: var(--bg-light);
    }
    &:active {
      background-color: var(--bg-light);
      &:not(.parent-dragged) {
        background-color: var(--bg);
        opacity: 0.3;
      }
    }
  }
</style>
