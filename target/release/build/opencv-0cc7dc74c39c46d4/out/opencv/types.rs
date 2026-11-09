
#[cfg(ocvrs_has_module_core)]
mod core_types {
	use crate::{mod_prelude::*, core, types, sys};

	impl core::GpuMat_AllocatorTraitConst for types::AbstractRefMut<'static, core::GpuMat_Allocator> {
		#[inline] fn as_raw_GpuMat_Allocator(&self) -> extern_send!(Self) { self.as_raw() }
	}

	impl core::GpuMat_AllocatorTrait for types::AbstractRefMut<'static, core::GpuMat_Allocator> {
		#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<core::Algorithm>` instead, removal in Nov 2024"]
	pub type PtrOfAlgorithm = core::Ptr<core::Algorithm>;

	ptr_extern! { core::Algorithm,
		cv_PtrLcv_AlgorithmG_new_null_const, cv_PtrLcv_AlgorithmG_delete, cv_PtrLcv_AlgorithmG_getInnerPtr_const, cv_PtrLcv_AlgorithmG_getInnerPtrMut
	}

	ptr_extern_ctor! { core::Algorithm, cv_PtrLcv_AlgorithmG_new_const_Algorithm }
	impl core::Ptr<core::Algorithm> {
		#[inline] pub fn as_raw_PtrOfAlgorithm(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAlgorithm(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<core::Algorithm> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<core::Algorithm> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::Algorithm> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAlgorithm")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<core::ConjGradSolver>` instead, removal in Nov 2024"]
	pub type PtrOfConjGradSolver = core::Ptr<core::ConjGradSolver>;

	ptr_extern! { core::ConjGradSolver,
		cv_PtrLcv_ConjGradSolverG_new_null_const, cv_PtrLcv_ConjGradSolverG_delete, cv_PtrLcv_ConjGradSolverG_getInnerPtr_const, cv_PtrLcv_ConjGradSolverG_getInnerPtrMut
	}

	impl core::Ptr<core::ConjGradSolver> {
		#[inline] pub fn as_raw_PtrOfConjGradSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConjGradSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::ConjGradSolverTraitConst for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_ConjGradSolver(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::ConjGradSolverTrait for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_mut_ConjGradSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<core::ConjGradSolver>, core::Ptr<core::Algorithm>, cv_PtrLcv_ConjGradSolverG_to_PtrOfAlgorithm }

	impl core::MinProblemSolverTraitConst for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::MinProblemSolverTrait for core::Ptr<core::ConjGradSolver> {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<core::ConjGradSolver>, core::Ptr<core::MinProblemSolver>, cv_PtrLcv_ConjGradSolverG_to_PtrOfMinProblemSolver }

	impl std::fmt::Debug for core::Ptr<core::ConjGradSolver> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfConjGradSolver")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<core::DownhillSolver>` instead, removal in Nov 2024"]
	pub type PtrOfDownhillSolver = core::Ptr<core::DownhillSolver>;

	ptr_extern! { core::DownhillSolver,
		cv_PtrLcv_DownhillSolverG_new_null_const, cv_PtrLcv_DownhillSolverG_delete, cv_PtrLcv_DownhillSolverG_getInnerPtr_const, cv_PtrLcv_DownhillSolverG_getInnerPtrMut
	}

	impl core::Ptr<core::DownhillSolver> {
		#[inline] pub fn as_raw_PtrOfDownhillSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDownhillSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::DownhillSolverTraitConst for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_DownhillSolver(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::DownhillSolverTrait for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_mut_DownhillSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<core::DownhillSolver>, core::Ptr<core::Algorithm>, cv_PtrLcv_DownhillSolverG_to_PtrOfAlgorithm }

	impl core::MinProblemSolverTraitConst for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::MinProblemSolverTrait for core::Ptr<core::DownhillSolver> {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<core::DownhillSolver>, core::Ptr<core::MinProblemSolver>, cv_PtrLcv_DownhillSolverG_to_PtrOfMinProblemSolver }

	impl std::fmt::Debug for core::Ptr<core::DownhillSolver> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDownhillSolver")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<core::FileStorage>` instead, removal in Nov 2024"]
	pub type PtrOfFileStorage = core::Ptr<core::FileStorage>;

	ptr_extern! { core::FileStorage,
		cv_PtrLcv_FileStorageG_new_null_const, cv_PtrLcv_FileStorageG_delete, cv_PtrLcv_FileStorageG_getInnerPtr_const, cv_PtrLcv_FileStorageG_getInnerPtrMut
	}

	ptr_extern_ctor! { core::FileStorage, cv_PtrLcv_FileStorageG_new_const_FileStorage }
	impl core::Ptr<core::FileStorage> {
		#[inline] pub fn as_raw_PtrOfFileStorage(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFileStorage(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::FileStorageTraitConst for core::Ptr<core::FileStorage> {
		#[inline] fn as_raw_FileStorage(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::FileStorageTrait for core::Ptr<core::FileStorage> {
		#[inline] fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::FileStorage> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFileStorage")
				.field("state", &core::FileStorageTraitConst::state(self))
				.field("elname", &core::FileStorageTraitConst::elname(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<core::Formatted>` instead, removal in Nov 2024"]
	pub type PtrOfFormatted = core::Ptr<core::Formatted>;

	ptr_extern! { core::Formatted,
		cv_PtrLcv_FormattedG_new_null_const, cv_PtrLcv_FormattedG_delete, cv_PtrLcv_FormattedG_getInnerPtr_const, cv_PtrLcv_FormattedG_getInnerPtrMut
	}

	impl core::Ptr<core::Formatted> {
		#[inline] pub fn as_raw_PtrOfFormatted(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatted(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::FormattedTraitConst for core::Ptr<core::Formatted> {
		#[inline] fn as_raw_Formatted(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::FormattedTrait for core::Ptr<core::Formatted> {
		#[inline] fn as_raw_mut_Formatted(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::Formatted> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFormatted")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<core::Formatter>` instead, removal in Nov 2024"]
	pub type PtrOfFormatter = core::Ptr<core::Formatter>;

	ptr_extern! { core::Formatter,
		cv_PtrLcv_FormatterG_new_null_const, cv_PtrLcv_FormatterG_delete, cv_PtrLcv_FormatterG_getInnerPtr_const, cv_PtrLcv_FormatterG_getInnerPtrMut
	}

	impl core::Ptr<core::Formatter> {
		#[inline] pub fn as_raw_PtrOfFormatter(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFormatter(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::FormatterTraitConst for core::Ptr<core::Formatter> {
		#[inline] fn as_raw_Formatter(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::FormatterTrait for core::Ptr<core::Formatter> {
		#[inline] fn as_raw_mut_Formatter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::Formatter> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFormatter")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<core::GpuMat_Allocator>` instead, removal in Nov 2024"]
	pub type PtrOfGpuMat_Allocator = core::Ptr<core::GpuMat_Allocator>;

	ptr_extern! { core::GpuMat_Allocator,
		cv_PtrLcv_cuda_GpuMat_AllocatorG_new_null_const, cv_PtrLcv_cuda_GpuMat_AllocatorG_delete, cv_PtrLcv_cuda_GpuMat_AllocatorG_getInnerPtr_const, cv_PtrLcv_cuda_GpuMat_AllocatorG_getInnerPtrMut
	}

	impl core::Ptr<core::GpuMat_Allocator> {
		#[inline] pub fn as_raw_PtrOfGpuMat_Allocator(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGpuMat_Allocator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::GpuMat_AllocatorTraitConst for core::Ptr<core::GpuMat_Allocator> {
		#[inline] fn as_raw_GpuMat_Allocator(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::GpuMat_AllocatorTrait for core::Ptr<core::GpuMat_Allocator> {
		#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::GpuMat_Allocator> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGpuMat_Allocator")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<core::MinProblemSolver>` instead, removal in Nov 2024"]
	pub type PtrOfMinProblemSolver = core::Ptr<core::MinProblemSolver>;

	ptr_extern! { core::MinProblemSolver,
		cv_PtrLcv_MinProblemSolverG_new_null_const, cv_PtrLcv_MinProblemSolverG_delete, cv_PtrLcv_MinProblemSolverG_getInnerPtr_const, cv_PtrLcv_MinProblemSolverG_getInnerPtrMut
	}

	impl core::Ptr<core::MinProblemSolver> {
		#[inline] pub fn as_raw_PtrOfMinProblemSolver(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::MinProblemSolverTraitConst for core::Ptr<core::MinProblemSolver> {
		#[inline] fn as_raw_MinProblemSolver(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::MinProblemSolverTrait for core::Ptr<core::MinProblemSolver> {
		#[inline] fn as_raw_mut_MinProblemSolver(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<core::MinProblemSolver> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<core::MinProblemSolver> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<core::MinProblemSolver>, core::Ptr<core::Algorithm>, cv_PtrLcv_MinProblemSolverG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<core::MinProblemSolver> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMinProblemSolver")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<core::MinProblemSolver_Function>` instead, removal in Nov 2024"]
	pub type PtrOfMinProblemSolver_Function = core::Ptr<core::MinProblemSolver_Function>;

	ptr_extern! { core::MinProblemSolver_Function,
		cv_PtrLcv_MinProblemSolver_FunctionG_new_null_const, cv_PtrLcv_MinProblemSolver_FunctionG_delete, cv_PtrLcv_MinProblemSolver_FunctionG_getInnerPtr_const, cv_PtrLcv_MinProblemSolver_FunctionG_getInnerPtrMut
	}

	impl core::Ptr<core::MinProblemSolver_Function> {
		#[inline] pub fn as_raw_PtrOfMinProblemSolver_Function(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMinProblemSolver_Function(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::MinProblemSolver_FunctionTraitConst for core::Ptr<core::MinProblemSolver_Function> {
		#[inline] fn as_raw_MinProblemSolver_Function(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::MinProblemSolver_FunctionTrait for core::Ptr<core::MinProblemSolver_Function> {
		#[inline] fn as_raw_mut_MinProblemSolver_Function(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::MinProblemSolver_Function> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMinProblemSolver_Function")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<core::OriginalClassName>` instead, removal in Nov 2024"]
	pub type PtrOfOriginalClassName = core::Ptr<core::OriginalClassName>;

	ptr_extern! { core::OriginalClassName,
		cv_PtrLcv_utils_nested_OriginalClassNameG_new_null_const, cv_PtrLcv_utils_nested_OriginalClassNameG_delete, cv_PtrLcv_utils_nested_OriginalClassNameG_getInnerPtr_const, cv_PtrLcv_utils_nested_OriginalClassNameG_getInnerPtrMut
	}

	ptr_extern_ctor! { core::OriginalClassName, cv_PtrLcv_utils_nested_OriginalClassNameG_new_const_OriginalClassName }
	impl core::Ptr<core::OriginalClassName> {
		#[inline] pub fn as_raw_PtrOfOriginalClassName(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfOriginalClassName(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl core::OriginalClassNameTraitConst for core::Ptr<core::OriginalClassName> {
		#[inline] fn as_raw_OriginalClassName(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::OriginalClassNameTrait for core::Ptr<core::OriginalClassName> {
		#[inline] fn as_raw_mut_OriginalClassName(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<core::OriginalClassName> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfOriginalClassName")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<f32>` instead, removal in Nov 2024"]
	pub type PtrOff32 = core::Ptr<f32>;

	ptr_extern! { f32,
		cv_PtrLfloatG_new_null_const, cv_PtrLfloatG_delete, cv_PtrLfloatG_getInnerPtr_const, cv_PtrLfloatG_getInnerPtrMut
	}

	ptr_extern_ctor! { f32, cv_PtrLfloatG_new_const_float }
	impl core::Ptr<f32> {
		#[inline] pub fn as_raw_PtrOff32(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	#[deprecated = "Use the the non-alias form `core::Tuple<(i32, f32)>` instead, removal in Nov 2024"]
	pub type TupleOfi32_f32 = core::Tuple<(i32, f32)>;

	impl core::Tuple<(i32, f32)> {
		pub fn as_raw_TupleOfi32_f32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfi32_f32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	tuple_extern! { (i32, f32),
		std_pairLint__floatG_new_const_int_float, std_pairLint__floatG_delete,
		0 = arg: i32, get_0 via std_pairLint__floatG_get_0_const,
		1 = arg_1: f32, get_1 via std_pairLint__floatG_get_1_const
	}

	#[deprecated = "Use the the non-alias form `core::Vector<core::DMatch>` instead, removal in Nov 2024"]
	pub type VectorOfDMatch = core::Vector<core::DMatch>;

	impl core::Vector<core::DMatch> {
		pub fn as_raw_VectorOfDMatch(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfDMatch(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::DMatch,
		std_vectorLcv_DMatchG_new_const, std_vectorLcv_DMatchG_delete,
		std_vectorLcv_DMatchG_len_const, std_vectorLcv_DMatchG_isEmpty_const,
		std_vectorLcv_DMatchG_capacity_const, std_vectorLcv_DMatchG_shrinkToFit,
		std_vectorLcv_DMatchG_reserve_size_t, std_vectorLcv_DMatchG_remove_size_t,
		std_vectorLcv_DMatchG_swap_size_t_size_t, std_vectorLcv_DMatchG_clear,
		std_vectorLcv_DMatchG_get_const_size_t, std_vectorLcv_DMatchG_set_size_t_const_DMatch,
		std_vectorLcv_DMatchG_push_const_DMatch, std_vectorLcv_DMatchG_insert_size_t_const_DMatch,
	}

	vector_copy_non_bool! { core::DMatch,
		std_vectorLcv_DMatchG_data_const, std_vectorLcv_DMatchG_dataMut, cv_fromSlice_const_const_DMatchX_size_t,
		std_vectorLcv_DMatchG_clone_const,
	}


	#[deprecated = "Use the the non-alias form `core::Vector<core::KeyPoint>` instead, removal in Nov 2024"]
	pub type VectorOfKeyPoint = core::Vector<core::KeyPoint>;

	impl core::Vector<core::KeyPoint> {
		pub fn as_raw_VectorOfKeyPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfKeyPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::KeyPoint,
		std_vectorLcv_KeyPointG_new_const, std_vectorLcv_KeyPointG_delete,
		std_vectorLcv_KeyPointG_len_const, std_vectorLcv_KeyPointG_isEmpty_const,
		std_vectorLcv_KeyPointG_capacity_const, std_vectorLcv_KeyPointG_shrinkToFit,
		std_vectorLcv_KeyPointG_reserve_size_t, std_vectorLcv_KeyPointG_remove_size_t,
		std_vectorLcv_KeyPointG_swap_size_t_size_t, std_vectorLcv_KeyPointG_clear,
		std_vectorLcv_KeyPointG_get_const_size_t, std_vectorLcv_KeyPointG_set_size_t_const_KeyPoint,
		std_vectorLcv_KeyPointG_push_const_KeyPoint, std_vectorLcv_KeyPointG_insert_size_t_const_KeyPoint,
	}

	vector_non_copy_or_bool! { clone core::KeyPoint }

	vector_boxed_ref! { core::KeyPoint }

	vector_extern! { BoxedRef<'t, core::KeyPoint>,
		std_vectorLcv_KeyPointG_new_const, std_vectorLcv_KeyPointG_delete,
		std_vectorLcv_KeyPointG_len_const, std_vectorLcv_KeyPointG_isEmpty_const,
		std_vectorLcv_KeyPointG_capacity_const, std_vectorLcv_KeyPointG_shrinkToFit,
		std_vectorLcv_KeyPointG_reserve_size_t, std_vectorLcv_KeyPointG_remove_size_t,
		std_vectorLcv_KeyPointG_swap_size_t_size_t, std_vectorLcv_KeyPointG_clear,
		std_vectorLcv_KeyPointG_get_const_size_t, std_vectorLcv_KeyPointG_set_size_t_const_KeyPoint,
		std_vectorLcv_KeyPointG_push_const_KeyPoint, std_vectorLcv_KeyPointG_insert_size_t_const_KeyPoint,
	}


	#[deprecated = "Use the the non-alias form `core::Vector<core::Mat>` instead, removal in Nov 2024"]
	pub type VectorOfMat = core::Vector<core::Mat>;

	impl core::Vector<core::Mat> {
		pub fn as_raw_VectorOfMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Mat,
		std_vectorLcv_MatG_new_const, std_vectorLcv_MatG_delete,
		std_vectorLcv_MatG_len_const, std_vectorLcv_MatG_isEmpty_const,
		std_vectorLcv_MatG_capacity_const, std_vectorLcv_MatG_shrinkToFit,
		std_vectorLcv_MatG_reserve_size_t, std_vectorLcv_MatG_remove_size_t,
		std_vectorLcv_MatG_swap_size_t_size_t, std_vectorLcv_MatG_clear,
		std_vectorLcv_MatG_get_const_size_t, std_vectorLcv_MatG_set_size_t_const_Mat,
		std_vectorLcv_MatG_push_const_Mat, std_vectorLcv_MatG_insert_size_t_const_Mat,
	}

	vector_non_copy_or_bool! { clone core::Mat }

	vector_boxed_ref! { core::Mat }

	vector_extern! { BoxedRef<'t, core::Mat>,
		std_vectorLcv_MatG_new_const, std_vectorLcv_MatG_delete,
		std_vectorLcv_MatG_len_const, std_vectorLcv_MatG_isEmpty_const,
		std_vectorLcv_MatG_capacity_const, std_vectorLcv_MatG_shrinkToFit,
		std_vectorLcv_MatG_reserve_size_t, std_vectorLcv_MatG_remove_size_t,
		std_vectorLcv_MatG_swap_size_t_size_t, std_vectorLcv_MatG_clear,
		std_vectorLcv_MatG_get_const_size_t, std_vectorLcv_MatG_set_size_t_const_Mat,
		std_vectorLcv_MatG_push_const_Mat, std_vectorLcv_MatG_insert_size_t_const_Mat,
	}


	#[deprecated = "Use the the non-alias form `core::Vector<core::PlatformInfo>` instead, removal in Nov 2024"]
	pub type VectorOfPlatformInfo = core::Vector<core::PlatformInfo>;

	impl core::Vector<core::PlatformInfo> {
		pub fn as_raw_VectorOfPlatformInfo(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPlatformInfo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::PlatformInfo,
		std_vectorLcv_ocl_PlatformInfoG_new_const, std_vectorLcv_ocl_PlatformInfoG_delete,
		std_vectorLcv_ocl_PlatformInfoG_len_const, std_vectorLcv_ocl_PlatformInfoG_isEmpty_const,
		std_vectorLcv_ocl_PlatformInfoG_capacity_const, std_vectorLcv_ocl_PlatformInfoG_shrinkToFit,
		std_vectorLcv_ocl_PlatformInfoG_reserve_size_t, std_vectorLcv_ocl_PlatformInfoG_remove_size_t,
		std_vectorLcv_ocl_PlatformInfoG_swap_size_t_size_t, std_vectorLcv_ocl_PlatformInfoG_clear,
		std_vectorLcv_ocl_PlatformInfoG_get_const_size_t, std_vectorLcv_ocl_PlatformInfoG_set_size_t_const_PlatformInfo,
		std_vectorLcv_ocl_PlatformInfoG_push_const_PlatformInfo, std_vectorLcv_ocl_PlatformInfoG_insert_size_t_const_PlatformInfo,
	}

	vector_non_copy_or_bool! { core::PlatformInfo }

	vector_boxed_ref! { core::PlatformInfo }

	vector_extern! { BoxedRef<'t, core::PlatformInfo>,
		std_vectorLcv_ocl_PlatformInfoG_new_const, std_vectorLcv_ocl_PlatformInfoG_delete,
		std_vectorLcv_ocl_PlatformInfoG_len_const, std_vectorLcv_ocl_PlatformInfoG_isEmpty_const,
		std_vectorLcv_ocl_PlatformInfoG_capacity_const, std_vectorLcv_ocl_PlatformInfoG_shrinkToFit,
		std_vectorLcv_ocl_PlatformInfoG_reserve_size_t, std_vectorLcv_ocl_PlatformInfoG_remove_size_t,
		std_vectorLcv_ocl_PlatformInfoG_swap_size_t_size_t, std_vectorLcv_ocl_PlatformInfoG_clear,
		std_vectorLcv_ocl_PlatformInfoG_get_const_size_t, std_vectorLcv_ocl_PlatformInfoG_set_size_t_const_PlatformInfo,
		std_vectorLcv_ocl_PlatformInfoG_push_const_PlatformInfo, std_vectorLcv_ocl_PlatformInfoG_insert_size_t_const_PlatformInfo,
	}


	#[deprecated = "Use the the non-alias form `core::Vector<core::Point>` instead, removal in Nov 2024"]
	pub type VectorOfPoint = core::Vector<core::Point>;

	impl core::Vector<core::Point> {
		pub fn as_raw_VectorOfPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Point,
		std_vectorLcv_PointG_new_const, std_vectorLcv_PointG_delete,
		std_vectorLcv_PointG_len_const, std_vectorLcv_PointG_isEmpty_const,
		std_vectorLcv_PointG_capacity_const, std_vectorLcv_PointG_shrinkToFit,
		std_vectorLcv_PointG_reserve_size_t, std_vectorLcv_PointG_remove_size_t,
		std_vectorLcv_PointG_swap_size_t_size_t, std_vectorLcv_PointG_clear,
		std_vectorLcv_PointG_get_const_size_t, std_vectorLcv_PointG_set_size_t_const_Point,
		std_vectorLcv_PointG_push_const_Point, std_vectorLcv_PointG_insert_size_t_const_Point,
	}

	vector_copy_non_bool! { core::Point,
		std_vectorLcv_PointG_data_const, std_vectorLcv_PointG_dataMut, cv_fromSlice_const_const_PointX_size_t,
		std_vectorLcv_PointG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Point> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_PointG_inputArray_const(self.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Point> }

	impl ToOutputArray for core::Vector<core::Point> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_PointG_outputArray(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Point> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_PointG_inputOutputArray(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Point> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Point2d>` instead, removal in Nov 2024"]
	pub type VectorOfPoint2d = core::Vector<core::Point2d>;

	impl core::Vector<core::Point2d> {
		pub fn as_raw_VectorOfPoint2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Point2d,
		std_vectorLcv_Point2dG_new_const, std_vectorLcv_Point2dG_delete,
		std_vectorLcv_Point2dG_len_const, std_vectorLcv_Point2dG_isEmpty_const,
		std_vectorLcv_Point2dG_capacity_const, std_vectorLcv_Point2dG_shrinkToFit,
		std_vectorLcv_Point2dG_reserve_size_t, std_vectorLcv_Point2dG_remove_size_t,
		std_vectorLcv_Point2dG_swap_size_t_size_t, std_vectorLcv_Point2dG_clear,
		std_vectorLcv_Point2dG_get_const_size_t, std_vectorLcv_Point2dG_set_size_t_const_Point2d,
		std_vectorLcv_Point2dG_push_const_Point2d, std_vectorLcv_Point2dG_insert_size_t_const_Point2d,
	}

	vector_copy_non_bool! { core::Point2d,
		std_vectorLcv_Point2dG_data_const, std_vectorLcv_Point2dG_dataMut, cv_fromSlice_const_const_Point2dX_size_t,
		std_vectorLcv_Point2dG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Point2d> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2dG_inputArray_const(self.as_raw_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Point2d> }

	impl ToOutputArray for core::Vector<core::Point2d> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2dG_outputArray(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Point2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2dG_inputOutputArray(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Point2d> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Point2f>` instead, removal in Nov 2024"]
	pub type VectorOfPoint2f = core::Vector<core::Point2f>;

	impl core::Vector<core::Point2f> {
		pub fn as_raw_VectorOfPoint2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPoint2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Point2f,
		std_vectorLcv_Point2fG_new_const, std_vectorLcv_Point2fG_delete,
		std_vectorLcv_Point2fG_len_const, std_vectorLcv_Point2fG_isEmpty_const,
		std_vectorLcv_Point2fG_capacity_const, std_vectorLcv_Point2fG_shrinkToFit,
		std_vectorLcv_Point2fG_reserve_size_t, std_vectorLcv_Point2fG_remove_size_t,
		std_vectorLcv_Point2fG_swap_size_t_size_t, std_vectorLcv_Point2fG_clear,
		std_vectorLcv_Point2fG_get_const_size_t, std_vectorLcv_Point2fG_set_size_t_const_Point2f,
		std_vectorLcv_Point2fG_push_const_Point2f, std_vectorLcv_Point2fG_insert_size_t_const_Point2f,
	}

	vector_copy_non_bool! { core::Point2f,
		std_vectorLcv_Point2fG_data_const, std_vectorLcv_Point2fG_dataMut, cv_fromSlice_const_const_Point2fX_size_t,
		std_vectorLcv_Point2fG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Point2f> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2fG_inputArray_const(self.as_raw_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Point2f> }

	impl ToOutputArray for core::Vector<core::Point2f> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2fG_outputArray(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Point2f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Point2fG_inputOutputArray(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Point2f> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Range>` instead, removal in Nov 2024"]
	pub type VectorOfRange = core::Vector<core::Range>;

	impl core::Vector<core::Range> {
		pub fn as_raw_VectorOfRange(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRange(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Range,
		std_vectorLcv_RangeG_new_const, std_vectorLcv_RangeG_delete,
		std_vectorLcv_RangeG_len_const, std_vectorLcv_RangeG_isEmpty_const,
		std_vectorLcv_RangeG_capacity_const, std_vectorLcv_RangeG_shrinkToFit,
		std_vectorLcv_RangeG_reserve_size_t, std_vectorLcv_RangeG_remove_size_t,
		std_vectorLcv_RangeG_swap_size_t_size_t, std_vectorLcv_RangeG_clear,
		std_vectorLcv_RangeG_get_const_size_t, std_vectorLcv_RangeG_set_size_t_const_Range,
		std_vectorLcv_RangeG_push_const_Range, std_vectorLcv_RangeG_insert_size_t_const_Range,
	}

	vector_non_copy_or_bool! { core::Range }

	vector_boxed_ref! { core::Range }

	vector_extern! { BoxedRef<'t, core::Range>,
		std_vectorLcv_RangeG_new_const, std_vectorLcv_RangeG_delete,
		std_vectorLcv_RangeG_len_const, std_vectorLcv_RangeG_isEmpty_const,
		std_vectorLcv_RangeG_capacity_const, std_vectorLcv_RangeG_shrinkToFit,
		std_vectorLcv_RangeG_reserve_size_t, std_vectorLcv_RangeG_remove_size_t,
		std_vectorLcv_RangeG_swap_size_t_size_t, std_vectorLcv_RangeG_clear,
		std_vectorLcv_RangeG_get_const_size_t, std_vectorLcv_RangeG_set_size_t_const_Range,
		std_vectorLcv_RangeG_push_const_Range, std_vectorLcv_RangeG_insert_size_t_const_Range,
	}


	#[deprecated = "Use the the non-alias form `core::Vector<core::Rect>` instead, removal in Nov 2024"]
	pub type VectorOfRect = core::Vector<core::Rect>;

	impl core::Vector<core::Rect> {
		pub fn as_raw_VectorOfRect(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Rect,
		std_vectorLcv_RectG_new_const, std_vectorLcv_RectG_delete,
		std_vectorLcv_RectG_len_const, std_vectorLcv_RectG_isEmpty_const,
		std_vectorLcv_RectG_capacity_const, std_vectorLcv_RectG_shrinkToFit,
		std_vectorLcv_RectG_reserve_size_t, std_vectorLcv_RectG_remove_size_t,
		std_vectorLcv_RectG_swap_size_t_size_t, std_vectorLcv_RectG_clear,
		std_vectorLcv_RectG_get_const_size_t, std_vectorLcv_RectG_set_size_t_const_Rect,
		std_vectorLcv_RectG_push_const_Rect, std_vectorLcv_RectG_insert_size_t_const_Rect,
	}

	vector_copy_non_bool! { core::Rect,
		std_vectorLcv_RectG_data_const, std_vectorLcv_RectG_dataMut, cv_fromSlice_const_const_RectX_size_t,
		std_vectorLcv_RectG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Rect> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_RectG_inputArray_const(self.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Rect> }

	impl ToOutputArray for core::Vector<core::Rect> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_RectG_outputArray(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Rect> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_RectG_inputOutputArray(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Rect> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::RotatedRect>` instead, removal in Nov 2024"]
	pub type VectorOfRotatedRect = core::Vector<core::RotatedRect>;

	impl core::Vector<core::RotatedRect> {
		pub fn as_raw_VectorOfRotatedRect(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfRotatedRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::RotatedRect,
		std_vectorLcv_RotatedRectG_new_const, std_vectorLcv_RotatedRectG_delete,
		std_vectorLcv_RotatedRectG_len_const, std_vectorLcv_RotatedRectG_isEmpty_const,
		std_vectorLcv_RotatedRectG_capacity_const, std_vectorLcv_RotatedRectG_shrinkToFit,
		std_vectorLcv_RotatedRectG_reserve_size_t, std_vectorLcv_RotatedRectG_remove_size_t,
		std_vectorLcv_RotatedRectG_swap_size_t_size_t, std_vectorLcv_RotatedRectG_clear,
		std_vectorLcv_RotatedRectG_get_const_size_t, std_vectorLcv_RotatedRectG_set_size_t_const_RotatedRect,
		std_vectorLcv_RotatedRectG_push_const_RotatedRect, std_vectorLcv_RotatedRectG_insert_size_t_const_RotatedRect,
	}

	vector_copy_non_bool! { core::RotatedRect,
		std_vectorLcv_RotatedRectG_data_const, std_vectorLcv_RotatedRectG_dataMut, cv_fromSlice_const_const_RotatedRectX_size_t,
		std_vectorLcv_RotatedRectG_clone_const,
	}


	#[deprecated = "Use the the non-alias form `core::Vector<String>` instead, removal in Nov 2024"]
	pub type VectorOfString = core::Vector<String>;

	impl core::Vector<String> {
		pub fn as_raw_VectorOfString(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfString(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { String,
		std_vectorLcv_StringG_new_const, std_vectorLcv_StringG_delete,
		std_vectorLcv_StringG_len_const, std_vectorLcv_StringG_isEmpty_const,
		std_vectorLcv_StringG_capacity_const, std_vectorLcv_StringG_shrinkToFit,
		std_vectorLcv_StringG_reserve_size_t, std_vectorLcv_StringG_remove_size_t,
		std_vectorLcv_StringG_swap_size_t_size_t, std_vectorLcv_StringG_clear,
		std_vectorLcv_StringG_get_const_size_t, std_vectorLcv_StringG_set_size_t_const_String,
		std_vectorLcv_StringG_push_const_String, std_vectorLcv_StringG_insert_size_t_const_String,
	}

	vector_non_copy_or_bool! { String }


	#[deprecated = "Use the the non-alias form `core::Vector<core::Vec2d>` instead, removal in Nov 2024"]
	pub type VectorOfVec2d = core::Vector<core::Vec2d>;

	impl core::Vector<core::Vec2d> {
		pub fn as_raw_VectorOfVec2d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec2d,
		std_vectorLcv_Vec2dG_new_const, std_vectorLcv_Vec2dG_delete,
		std_vectorLcv_Vec2dG_len_const, std_vectorLcv_Vec2dG_isEmpty_const,
		std_vectorLcv_Vec2dG_capacity_const, std_vectorLcv_Vec2dG_shrinkToFit,
		std_vectorLcv_Vec2dG_reserve_size_t, std_vectorLcv_Vec2dG_remove_size_t,
		std_vectorLcv_Vec2dG_swap_size_t_size_t, std_vectorLcv_Vec2dG_clear,
		std_vectorLcv_Vec2dG_get_const_size_t, std_vectorLcv_Vec2dG_set_size_t_const_Vec2d,
		std_vectorLcv_Vec2dG_push_const_Vec2d, std_vectorLcv_Vec2dG_insert_size_t_const_Vec2d,
	}

	vector_copy_non_bool! { core::Vec2d,
		std_vectorLcv_Vec2dG_data_const, std_vectorLcv_Vec2dG_dataMut, cv_fromSlice_const_const_Vec2dX_size_t,
		std_vectorLcv_Vec2dG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2dG_inputArray_const(self.as_raw_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec2d> }

	impl ToOutputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2dG_outputArray(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec2d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2dG_inputOutputArray(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec2d> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Vec2f>` instead, removal in Nov 2024"]
	pub type VectorOfVec2f = core::Vector<core::Vec2f>;

	impl core::Vector<core::Vec2f> {
		pub fn as_raw_VectorOfVec2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec2f,
		std_vectorLcv_Vec2fG_new_const, std_vectorLcv_Vec2fG_delete,
		std_vectorLcv_Vec2fG_len_const, std_vectorLcv_Vec2fG_isEmpty_const,
		std_vectorLcv_Vec2fG_capacity_const, std_vectorLcv_Vec2fG_shrinkToFit,
		std_vectorLcv_Vec2fG_reserve_size_t, std_vectorLcv_Vec2fG_remove_size_t,
		std_vectorLcv_Vec2fG_swap_size_t_size_t, std_vectorLcv_Vec2fG_clear,
		std_vectorLcv_Vec2fG_get_const_size_t, std_vectorLcv_Vec2fG_set_size_t_const_Vec2f,
		std_vectorLcv_Vec2fG_push_const_Vec2f, std_vectorLcv_Vec2fG_insert_size_t_const_Vec2f,
	}

	vector_copy_non_bool! { core::Vec2f,
		std_vectorLcv_Vec2fG_data_const, std_vectorLcv_Vec2fG_dataMut, cv_fromSlice_const_const_Vec2fX_size_t,
		std_vectorLcv_Vec2fG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2fG_inputArray_const(self.as_raw_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec2f> }

	impl ToOutputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2fG_outputArray(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec2f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec2fG_inputOutputArray(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec2f> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Vec3d>` instead, removal in Nov 2024"]
	pub type VectorOfVec3d = core::Vector<core::Vec3d>;

	impl core::Vector<core::Vec3d> {
		pub fn as_raw_VectorOfVec3d(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec3d,
		std_vectorLcv_Vec3dG_new_const, std_vectorLcv_Vec3dG_delete,
		std_vectorLcv_Vec3dG_len_const, std_vectorLcv_Vec3dG_isEmpty_const,
		std_vectorLcv_Vec3dG_capacity_const, std_vectorLcv_Vec3dG_shrinkToFit,
		std_vectorLcv_Vec3dG_reserve_size_t, std_vectorLcv_Vec3dG_remove_size_t,
		std_vectorLcv_Vec3dG_swap_size_t_size_t, std_vectorLcv_Vec3dG_clear,
		std_vectorLcv_Vec3dG_get_const_size_t, std_vectorLcv_Vec3dG_set_size_t_const_Vec3d,
		std_vectorLcv_Vec3dG_push_const_Vec3d, std_vectorLcv_Vec3dG_insert_size_t_const_Vec3d,
	}

	vector_copy_non_bool! { core::Vec3d,
		std_vectorLcv_Vec3dG_data_const, std_vectorLcv_Vec3dG_dataMut, cv_fromSlice_const_const_Vec3dX_size_t,
		std_vectorLcv_Vec3dG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3dG_inputArray_const(self.as_raw_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec3d> }

	impl ToOutputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3dG_outputArray(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec3d> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3dG_inputOutputArray(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec3d> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Vec3f>` instead, removal in Nov 2024"]
	pub type VectorOfVec3f = core::Vector<core::Vec3f>;

	impl core::Vector<core::Vec3f> {
		pub fn as_raw_VectorOfVec3f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec3f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec3f,
		std_vectorLcv_Vec3fG_new_const, std_vectorLcv_Vec3fG_delete,
		std_vectorLcv_Vec3fG_len_const, std_vectorLcv_Vec3fG_isEmpty_const,
		std_vectorLcv_Vec3fG_capacity_const, std_vectorLcv_Vec3fG_shrinkToFit,
		std_vectorLcv_Vec3fG_reserve_size_t, std_vectorLcv_Vec3fG_remove_size_t,
		std_vectorLcv_Vec3fG_swap_size_t_size_t, std_vectorLcv_Vec3fG_clear,
		std_vectorLcv_Vec3fG_get_const_size_t, std_vectorLcv_Vec3fG_set_size_t_const_Vec3f,
		std_vectorLcv_Vec3fG_push_const_Vec3f, std_vectorLcv_Vec3fG_insert_size_t_const_Vec3f,
	}

	vector_copy_non_bool! { core::Vec3f,
		std_vectorLcv_Vec3fG_data_const, std_vectorLcv_Vec3fG_dataMut, cv_fromSlice_const_const_Vec3fX_size_t,
		std_vectorLcv_Vec3fG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3fG_inputArray_const(self.as_raw_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec3f> }

	impl ToOutputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3fG_outputArray(self.as_raw_mut_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec3f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec3fG_inputOutputArray(self.as_raw_mut_VectorOfVec3f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec3f> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Vec4f>` instead, removal in Nov 2024"]
	pub type VectorOfVec4f = core::Vector<core::Vec4f>;

	impl core::Vector<core::Vec4f> {
		pub fn as_raw_VectorOfVec4f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec4f,
		std_vectorLcv_Vec4fG_new_const, std_vectorLcv_Vec4fG_delete,
		std_vectorLcv_Vec4fG_len_const, std_vectorLcv_Vec4fG_isEmpty_const,
		std_vectorLcv_Vec4fG_capacity_const, std_vectorLcv_Vec4fG_shrinkToFit,
		std_vectorLcv_Vec4fG_reserve_size_t, std_vectorLcv_Vec4fG_remove_size_t,
		std_vectorLcv_Vec4fG_swap_size_t_size_t, std_vectorLcv_Vec4fG_clear,
		std_vectorLcv_Vec4fG_get_const_size_t, std_vectorLcv_Vec4fG_set_size_t_const_Vec4f,
		std_vectorLcv_Vec4fG_push_const_Vec4f, std_vectorLcv_Vec4fG_insert_size_t_const_Vec4f,
	}

	vector_copy_non_bool! { core::Vec4f,
		std_vectorLcv_Vec4fG_data_const, std_vectorLcv_Vec4fG_dataMut, cv_fromSlice_const_const_Vec4fX_size_t,
		std_vectorLcv_Vec4fG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4fG_inputArray_const(self.as_raw_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec4f> }

	impl ToOutputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4fG_outputArray(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec4f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4fG_inputOutputArray(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec4f> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Vec4i>` instead, removal in Nov 2024"]
	pub type VectorOfVec4i = core::Vector<core::Vec4i>;

	impl core::Vector<core::Vec4i> {
		pub fn as_raw_VectorOfVec4i(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec4i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec4i,
		std_vectorLcv_Vec4iG_new_const, std_vectorLcv_Vec4iG_delete,
		std_vectorLcv_Vec4iG_len_const, std_vectorLcv_Vec4iG_isEmpty_const,
		std_vectorLcv_Vec4iG_capacity_const, std_vectorLcv_Vec4iG_shrinkToFit,
		std_vectorLcv_Vec4iG_reserve_size_t, std_vectorLcv_Vec4iG_remove_size_t,
		std_vectorLcv_Vec4iG_swap_size_t_size_t, std_vectorLcv_Vec4iG_clear,
		std_vectorLcv_Vec4iG_get_const_size_t, std_vectorLcv_Vec4iG_set_size_t_const_Vec4i,
		std_vectorLcv_Vec4iG_push_const_Vec4i, std_vectorLcv_Vec4iG_insert_size_t_const_Vec4i,
	}

	vector_copy_non_bool! { core::Vec4i,
		std_vectorLcv_Vec4iG_data_const, std_vectorLcv_Vec4iG_dataMut, cv_fromSlice_const_const_Vec4iX_size_t,
		std_vectorLcv_Vec4iG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4iG_inputArray_const(self.as_raw_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec4i> }

	impl ToOutputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4iG_outputArray(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec4i> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec4iG_inputOutputArray(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec4i> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Vec6f>` instead, removal in Nov 2024"]
	pub type VectorOfVec6f = core::Vector<core::Vec6f>;

	impl core::Vector<core::Vec6f> {
		pub fn as_raw_VectorOfVec6f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVec6f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vec6f,
		std_vectorLcv_Vec6fG_new_const, std_vectorLcv_Vec6fG_delete,
		std_vectorLcv_Vec6fG_len_const, std_vectorLcv_Vec6fG_isEmpty_const,
		std_vectorLcv_Vec6fG_capacity_const, std_vectorLcv_Vec6fG_shrinkToFit,
		std_vectorLcv_Vec6fG_reserve_size_t, std_vectorLcv_Vec6fG_remove_size_t,
		std_vectorLcv_Vec6fG_swap_size_t_size_t, std_vectorLcv_Vec6fG_clear,
		std_vectorLcv_Vec6fG_get_const_size_t, std_vectorLcv_Vec6fG_set_size_t_const_Vec6f,
		std_vectorLcv_Vec6fG_push_const_Vec6f, std_vectorLcv_Vec6fG_insert_size_t_const_Vec6f,
	}

	vector_copy_non_bool! { core::Vec6f,
		std_vectorLcv_Vec6fG_data_const, std_vectorLcv_Vec6fG_dataMut, cv_fromSlice_const_const_Vec6fX_size_t,
		std_vectorLcv_Vec6fG_clone_const,
	}

	impl ToInputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec6fG_inputArray_const(self.as_raw_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vec6f> }

	impl ToOutputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec6fG_outputArray(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vec6f> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLcv_Vec6fG_inputOutputArray(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vec6f> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Vector<core::Mat>>` instead, removal in Nov 2024"]
	pub type VectorOfVectorOfMat = core::Vector<core::Vector<core::Mat>>;

	impl core::Vector<core::Vector<core::Mat>> {
		pub fn as_raw_VectorOfVectorOfMat(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMat(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<core::Mat>,
		std_vectorLstd_vectorLcv_MatGG_new_const, std_vectorLstd_vectorLcv_MatGG_delete,
		std_vectorLstd_vectorLcv_MatGG_len_const, std_vectorLstd_vectorLcv_MatGG_isEmpty_const,
		std_vectorLstd_vectorLcv_MatGG_capacity_const, std_vectorLstd_vectorLcv_MatGG_shrinkToFit,
		std_vectorLstd_vectorLcv_MatGG_reserve_size_t, std_vectorLstd_vectorLcv_MatGG_remove_size_t,
		std_vectorLstd_vectorLcv_MatGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_MatGG_clear,
		std_vectorLstd_vectorLcv_MatGG_get_const_size_t, std_vectorLstd_vectorLcv_MatGG_set_size_t_const_vectorLMatG,
		std_vectorLstd_vectorLcv_MatGG_push_const_vectorLMatG, std_vectorLstd_vectorLcv_MatGG_insert_size_t_const_vectorLMatG,
	}

	vector_non_copy_or_bool! { clone core::Vector<core::Mat> }


	#[deprecated = "Use the the non-alias form `core::Vector<core::Vector<core::Point>>` instead, removal in Nov 2024"]
	pub type VectorOfVectorOfPoint = core::Vector<core::Vector<core::Point>>;

	impl core::Vector<core::Vector<core::Point>> {
		pub fn as_raw_VectorOfVectorOfPoint(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<core::Point>,
		std_vectorLstd_vectorLcv_PointGG_new_const, std_vectorLstd_vectorLcv_PointGG_delete,
		std_vectorLstd_vectorLcv_PointGG_len_const, std_vectorLstd_vectorLcv_PointGG_isEmpty_const,
		std_vectorLstd_vectorLcv_PointGG_capacity_const, std_vectorLstd_vectorLcv_PointGG_shrinkToFit,
		std_vectorLstd_vectorLcv_PointGG_reserve_size_t, std_vectorLstd_vectorLcv_PointGG_remove_size_t,
		std_vectorLstd_vectorLcv_PointGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_PointGG_clear,
		std_vectorLstd_vectorLcv_PointGG_get_const_size_t, std_vectorLstd_vectorLcv_PointGG_set_size_t_const_vectorLPointG,
		std_vectorLstd_vectorLcv_PointGG_push_const_vectorLPointG, std_vectorLstd_vectorLcv_PointGG_insert_size_t_const_vectorLPointG,
	}

	vector_non_copy_or_bool! { clone core::Vector<core::Point> }

	impl ToInputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_PointGG_inputArray_const(self.as_raw_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vector<core::Point>> }

	impl ToOutputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_PointGG_outputArray(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vector<core::Point>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_PointGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vector<core::Point>> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Vector<core::Point2f>>` instead, removal in Nov 2024"]
	pub type VectorOfVectorOfPoint2f = core::Vector<core::Vector<core::Point2f>>;

	impl core::Vector<core::Vector<core::Point2f>> {
		pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfPoint2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<core::Point2f>,
		std_vectorLstd_vectorLcv_Point2fGG_new_const, std_vectorLstd_vectorLcv_Point2fGG_delete,
		std_vectorLstd_vectorLcv_Point2fGG_len_const, std_vectorLstd_vectorLcv_Point2fGG_isEmpty_const,
		std_vectorLstd_vectorLcv_Point2fGG_capacity_const, std_vectorLstd_vectorLcv_Point2fGG_shrinkToFit,
		std_vectorLstd_vectorLcv_Point2fGG_reserve_size_t, std_vectorLstd_vectorLcv_Point2fGG_remove_size_t,
		std_vectorLstd_vectorLcv_Point2fGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_Point2fGG_clear,
		std_vectorLstd_vectorLcv_Point2fGG_get_const_size_t, std_vectorLstd_vectorLcv_Point2fGG_set_size_t_const_vectorLPoint2fG,
		std_vectorLstd_vectorLcv_Point2fGG_push_const_vectorLPoint2fG, std_vectorLstd_vectorLcv_Point2fGG_insert_size_t_const_vectorLPoint2fG,
	}

	vector_non_copy_or_bool! { clone core::Vector<core::Point2f> }

	impl ToInputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_Point2fGG_inputArray_const(self.as_raw_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vector<core::Point2f>> }

	impl ToOutputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_Point2fGG_outputArray(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vector<core::Point2f>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLcv_Point2fGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vector<core::Point2f>> }

	#[deprecated = "Use the the non-alias form `core::Vector<core::Vector<core::Range>>` instead, removal in Nov 2024"]
	pub type VectorOfVectorOfRange = core::Vector<core::Vector<core::Range>>;

	impl core::Vector<core::Vector<core::Range>> {
		pub fn as_raw_VectorOfVectorOfRange(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfRange(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<core::Range>,
		std_vectorLstd_vectorLcv_RangeGG_new_const, std_vectorLstd_vectorLcv_RangeGG_delete,
		std_vectorLstd_vectorLcv_RangeGG_len_const, std_vectorLstd_vectorLcv_RangeGG_isEmpty_const,
		std_vectorLstd_vectorLcv_RangeGG_capacity_const, std_vectorLstd_vectorLcv_RangeGG_shrinkToFit,
		std_vectorLstd_vectorLcv_RangeGG_reserve_size_t, std_vectorLstd_vectorLcv_RangeGG_remove_size_t,
		std_vectorLstd_vectorLcv_RangeGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_RangeGG_clear,
		std_vectorLstd_vectorLcv_RangeGG_get_const_size_t, std_vectorLstd_vectorLcv_RangeGG_set_size_t_const_vectorLRangeG,
		std_vectorLstd_vectorLcv_RangeGG_push_const_vectorLRangeG, std_vectorLstd_vectorLcv_RangeGG_insert_size_t_const_vectorLRangeG,
	}

	vector_non_copy_or_bool! { core::Vector<core::Range> }


	#[deprecated = "Use the the non-alias form `core::Vector<core::Vector<i32>>` instead, removal in Nov 2024"]
	pub type VectorOfVectorOfi32 = core::Vector<core::Vector<i32>>;

	impl core::Vector<core::Vector<i32>> {
		pub fn as_raw_VectorOfVectorOfi32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfi32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<i32>,
		std_vectorLstd_vectorLintGG_new_const, std_vectorLstd_vectorLintGG_delete,
		std_vectorLstd_vectorLintGG_len_const, std_vectorLstd_vectorLintGG_isEmpty_const,
		std_vectorLstd_vectorLintGG_capacity_const, std_vectorLstd_vectorLintGG_shrinkToFit,
		std_vectorLstd_vectorLintGG_reserve_size_t, std_vectorLstd_vectorLintGG_remove_size_t,
		std_vectorLstd_vectorLintGG_swap_size_t_size_t, std_vectorLstd_vectorLintGG_clear,
		std_vectorLstd_vectorLintGG_get_const_size_t, std_vectorLstd_vectorLintGG_set_size_t_const_vectorLintG,
		std_vectorLstd_vectorLintGG_push_const_vectorLintG, std_vectorLstd_vectorLintGG_insert_size_t_const_vectorLintG,
	}

	vector_non_copy_or_bool! { clone core::Vector<i32> }

	impl ToInputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLintGG_inputArray_const(self.as_raw_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<core::Vector<i32>> }

	impl ToOutputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLintGG_outputArray(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<core::Vector<i32>> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLstd_vectorLintGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<core::Vector<i32>> }

	#[deprecated = "Use the the non-alias form `core::Vector<bool>` instead, removal in Nov 2024"]
	pub type VectorOfbool = core::Vector<bool>;

	impl core::Vector<bool> {
		pub fn as_raw_VectorOfbool(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfbool(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { bool,
		std_vectorLboolG_new_const, std_vectorLboolG_delete,
		std_vectorLboolG_len_const, std_vectorLboolG_isEmpty_const,
		std_vectorLboolG_capacity_const, std_vectorLboolG_shrinkToFit,
		std_vectorLboolG_reserve_size_t, std_vectorLboolG_remove_size_t,
		std_vectorLboolG_swap_size_t_size_t, std_vectorLboolG_clear,
		std_vectorLboolG_get_const_size_t, std_vectorLboolG_set_size_t_const_bool,
		std_vectorLboolG_push_const_bool, std_vectorLboolG_insert_size_t_const_bool,
	}

	vector_non_copy_or_bool! { clone bool }

	impl ToInputArray for core::Vector<bool> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLboolG_inputArray_const(self.as_raw_VectorOfbool(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<bool> }

	#[deprecated = "Use the the non-alias form `core::Vector<c_char>` instead, removal in Nov 2024"]
	pub type VectorOfc_char = core::Vector<c_char>;

	impl core::Vector<c_char> {
		pub fn as_raw_VectorOfc_char(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfc_char(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	#[deprecated = "Use the the non-alias form `core::Vector<f32>` instead, removal in Nov 2024"]
	pub type VectorOff32 = core::Vector<f32>;

	impl core::Vector<f32> {
		pub fn as_raw_VectorOff32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOff32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { f32,
		std_vectorLfloatG_new_const, std_vectorLfloatG_delete,
		std_vectorLfloatG_len_const, std_vectorLfloatG_isEmpty_const,
		std_vectorLfloatG_capacity_const, std_vectorLfloatG_shrinkToFit,
		std_vectorLfloatG_reserve_size_t, std_vectorLfloatG_remove_size_t,
		std_vectorLfloatG_swap_size_t_size_t, std_vectorLfloatG_clear,
		std_vectorLfloatG_get_const_size_t, std_vectorLfloatG_set_size_t_const_float,
		std_vectorLfloatG_push_const_float, std_vectorLfloatG_insert_size_t_const_float,
	}

	vector_copy_non_bool! { f32,
		std_vectorLfloatG_data_const, std_vectorLfloatG_dataMut, cv_fromSlice_const_const_floatX_size_t,
		std_vectorLfloatG_clone_const,
	}

	impl ToInputArray for core::Vector<f32> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLfloatG_inputArray_const(self.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<f32> }

	impl ToOutputArray for core::Vector<f32> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLfloatG_outputArray(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<f32> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLfloatG_inputOutputArray(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<f32> }

	#[deprecated = "Use the the non-alias form `core::Vector<f64>` instead, removal in Nov 2024"]
	pub type VectorOff64 = core::Vector<f64>;

	impl core::Vector<f64> {
		pub fn as_raw_VectorOff64(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOff64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { f64,
		std_vectorLdoubleG_new_const, std_vectorLdoubleG_delete,
		std_vectorLdoubleG_len_const, std_vectorLdoubleG_isEmpty_const,
		std_vectorLdoubleG_capacity_const, std_vectorLdoubleG_shrinkToFit,
		std_vectorLdoubleG_reserve_size_t, std_vectorLdoubleG_remove_size_t,
		std_vectorLdoubleG_swap_size_t_size_t, std_vectorLdoubleG_clear,
		std_vectorLdoubleG_get_const_size_t, std_vectorLdoubleG_set_size_t_const_double,
		std_vectorLdoubleG_push_const_double, std_vectorLdoubleG_insert_size_t_const_double,
	}

	vector_copy_non_bool! { f64,
		std_vectorLdoubleG_data_const, std_vectorLdoubleG_dataMut, cv_fromSlice_const_const_doubleX_size_t,
		std_vectorLdoubleG_clone_const,
	}

	impl ToInputArray for core::Vector<f64> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLdoubleG_inputArray_const(self.as_raw_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<f64> }

	impl ToOutputArray for core::Vector<f64> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLdoubleG_outputArray(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<f64> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLdoubleG_inputOutputArray(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<f64> }

	#[deprecated = "Use the the non-alias form `core::Vector<i32>` instead, removal in Nov 2024"]
	pub type VectorOfi32 = core::Vector<i32>;

	impl core::Vector<i32> {
		pub fn as_raw_VectorOfi32(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfi32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { i32,
		std_vectorLintG_new_const, std_vectorLintG_delete,
		std_vectorLintG_len_const, std_vectorLintG_isEmpty_const,
		std_vectorLintG_capacity_const, std_vectorLintG_shrinkToFit,
		std_vectorLintG_reserve_size_t, std_vectorLintG_remove_size_t,
		std_vectorLintG_swap_size_t_size_t, std_vectorLintG_clear,
		std_vectorLintG_get_const_size_t, std_vectorLintG_set_size_t_const_int,
		std_vectorLintG_push_const_int, std_vectorLintG_insert_size_t_const_int,
	}

	vector_copy_non_bool! { i32,
		std_vectorLintG_data_const, std_vectorLintG_dataMut, cv_fromSlice_const_const_intX_size_t,
		std_vectorLintG_clone_const,
	}

	impl ToInputArray for core::Vector<i32> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLintG_inputArray_const(self.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<i32> }

	impl ToOutputArray for core::Vector<i32> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLintG_outputArray(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<i32> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLintG_inputOutputArray(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<i32> }

	#[deprecated = "Use the the non-alias form `core::Vector<i8>` instead, removal in Nov 2024"]
	pub type VectorOfi8 = core::Vector<i8>;

	impl core::Vector<i8> {
		pub fn as_raw_VectorOfi8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfi8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { i8,
		std_vectorLsigned_charG_new_const, std_vectorLsigned_charG_delete,
		std_vectorLsigned_charG_len_const, std_vectorLsigned_charG_isEmpty_const,
		std_vectorLsigned_charG_capacity_const, std_vectorLsigned_charG_shrinkToFit,
		std_vectorLsigned_charG_reserve_size_t, std_vectorLsigned_charG_remove_size_t,
		std_vectorLsigned_charG_swap_size_t_size_t, std_vectorLsigned_charG_clear,
		std_vectorLsigned_charG_get_const_size_t, std_vectorLsigned_charG_set_size_t_const_signed_char,
		std_vectorLsigned_charG_push_const_signed_char, std_vectorLsigned_charG_insert_size_t_const_signed_char,
	}

	vector_copy_non_bool! { i8,
		std_vectorLsigned_charG_data_const, std_vectorLsigned_charG_dataMut, cv_fromSlice_const_const_signed_charX_size_t,
		std_vectorLsigned_charG_clone_const,
	}


	#[deprecated = "Use the the non-alias form `core::Vector<size_t>` instead, removal in Nov 2024"]
	pub type VectorOfsize_t = core::Vector<size_t>;

	impl core::Vector<size_t> {
		pub fn as_raw_VectorOfsize_t(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfsize_t(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { size_t,
		std_vectorLsize_tG_new_const, std_vectorLsize_tG_delete,
		std_vectorLsize_tG_len_const, std_vectorLsize_tG_isEmpty_const,
		std_vectorLsize_tG_capacity_const, std_vectorLsize_tG_shrinkToFit,
		std_vectorLsize_tG_reserve_size_t, std_vectorLsize_tG_remove_size_t,
		std_vectorLsize_tG_swap_size_t_size_t, std_vectorLsize_tG_clear,
		std_vectorLsize_tG_get_const_size_t, std_vectorLsize_tG_set_size_t_const_size_t,
		std_vectorLsize_tG_push_const_size_t, std_vectorLsize_tG_insert_size_t_const_size_t,
	}

	vector_copy_non_bool! { size_t,
		std_vectorLsize_tG_data_const, std_vectorLsize_tG_dataMut, cv_fromSlice_const_const_size_tX_size_t,
		std_vectorLsize_tG_clone_const,
	}


	#[deprecated = "Use the the non-alias form `core::Vector<u8>` instead, removal in Nov 2024"]
	pub type VectorOfu8 = core::Vector<u8>;

	impl core::Vector<u8> {
		pub fn as_raw_VectorOfu8(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfu8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { u8,
		std_vectorLunsigned_charG_new_const, std_vectorLunsigned_charG_delete,
		std_vectorLunsigned_charG_len_const, std_vectorLunsigned_charG_isEmpty_const,
		std_vectorLunsigned_charG_capacity_const, std_vectorLunsigned_charG_shrinkToFit,
		std_vectorLunsigned_charG_reserve_size_t, std_vectorLunsigned_charG_remove_size_t,
		std_vectorLunsigned_charG_swap_size_t_size_t, std_vectorLunsigned_charG_clear,
		std_vectorLunsigned_charG_get_const_size_t, std_vectorLunsigned_charG_set_size_t_const_unsigned_char,
		std_vectorLunsigned_charG_push_const_unsigned_char, std_vectorLunsigned_charG_insert_size_t_const_unsigned_char,
	}

	vector_copy_non_bool! { u8,
		std_vectorLunsigned_charG_data_const, std_vectorLunsigned_charG_dataMut, cv_fromSlice_const_const_unsigned_charX_size_t,
		std_vectorLunsigned_charG_clone_const,
	}

	impl ToInputArray for core::Vector<u8> {
		#[inline]
		fn input_array(&self) -> Result<BoxedRef<core::_InputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLunsigned_charG_inputArray_const(self.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRef::<core::_InputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	input_array_ref_forward! { core::Vector<u8> }

	impl ToOutputArray for core::Vector<u8> {
		#[inline]
		fn output_array(&mut self) -> Result<BoxedRefMut<core::_OutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLunsigned_charG_outputArray(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_OutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	impl ToInputOutputArray for core::Vector<u8> {
		#[inline]
		fn input_output_array(&mut self) -> Result<BoxedRefMut<core::_InputOutputArray>> {
			return_send!(via ocvrs_return);
			unsafe { sys::std_vectorLunsigned_charG_inputOutputArray(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { BoxedRefMut::<core::_InputOutputArray>::opencv_from_extern(ret) };
			Ok(ret)
		}

	}

	output_array_ref_forward! { core::Vector<u8> }

}
#[cfg(ocvrs_has_module_core)]
pub use core_types::*;

#[cfg(ocvrs_has_module_dnn)]
mod dnn_types {
	use crate::{mod_prelude::*, core, types, sys};

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::AbsLayer>` instead, removal in Nov 2024"]
	pub type PtrOfAbsLayer = core::Ptr<crate::dnn::AbsLayer>;

	ptr_extern! { crate::dnn::AbsLayer,
		cv_PtrLcv_dnn_AbsLayerG_new_null_const, cv_PtrLcv_dnn_AbsLayerG_delete, cv_PtrLcv_dnn_AbsLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AbsLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AbsLayer, cv_PtrLcv_dnn_AbsLayerG_new_const_AbsLayer }
	impl core::Ptr<crate::dnn::AbsLayer> {
		#[inline] pub fn as_raw_PtrOfAbsLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAbsLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AbsLayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_AbsLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AbsLayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_AbsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AbsLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AbsLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AbsLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AbsLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AbsLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AbsLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AbsLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AbsLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAbsLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::AccumLayer>` instead, removal in Nov 2024"]
	pub type PtrOfAccumLayer = core::Ptr<crate::dnn::AccumLayer>;

	ptr_extern! { crate::dnn::AccumLayer,
		cv_PtrLcv_dnn_AccumLayerG_new_null_const, cv_PtrLcv_dnn_AccumLayerG_delete, cv_PtrLcv_dnn_AccumLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AccumLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AccumLayer, cv_PtrLcv_dnn_AccumLayerG_new_const_AccumLayer }
	impl core::Ptr<crate::dnn::AccumLayer> {
		#[inline] pub fn as_raw_PtrOfAccumLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAccumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AccumLayerTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_AccumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AccumLayerTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_AccumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AccumLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AccumLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AccumLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AccumLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AccumLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AccumLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAccumLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::AcosLayer>` instead, removal in Nov 2024"]
	pub type PtrOfAcosLayer = core::Ptr<crate::dnn::AcosLayer>;

	ptr_extern! { crate::dnn::AcosLayer,
		cv_PtrLcv_dnn_AcosLayerG_new_null_const, cv_PtrLcv_dnn_AcosLayerG_delete, cv_PtrLcv_dnn_AcosLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AcosLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AcosLayer, cv_PtrLcv_dnn_AcosLayerG_new_const_AcosLayer }
	impl core::Ptr<crate::dnn::AcosLayer> {
		#[inline] pub fn as_raw_PtrOfAcosLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAcosLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AcosLayerTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_AcosLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AcosLayerTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_AcosLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AcosLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AcosLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AcosLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AcosLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AcosLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AcosLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AcosLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AcosLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAcosLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::AcoshLayer>` instead, removal in Nov 2024"]
	pub type PtrOfAcoshLayer = core::Ptr<crate::dnn::AcoshLayer>;

	ptr_extern! { crate::dnn::AcoshLayer,
		cv_PtrLcv_dnn_AcoshLayerG_new_null_const, cv_PtrLcv_dnn_AcoshLayerG_delete, cv_PtrLcv_dnn_AcoshLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AcoshLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AcoshLayer, cv_PtrLcv_dnn_AcoshLayerG_new_const_AcoshLayer }
	impl core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] pub fn as_raw_PtrOfAcoshLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAcoshLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AcoshLayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_AcoshLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AcoshLayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_AcoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AcoshLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AcoshLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AcoshLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AcoshLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AcoshLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AcoshLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AcoshLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAcoshLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ActivationLayer>` instead, removal in Nov 2024"]
	pub type PtrOfActivationLayer = core::Ptr<crate::dnn::ActivationLayer>;

	ptr_extern! { crate::dnn::ActivationLayer,
		cv_PtrLcv_dnn_ActivationLayerG_new_null_const, cv_PtrLcv_dnn_ActivationLayerG_delete, cv_PtrLcv_dnn_ActivationLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ActivationLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ActivationLayer, cv_PtrLcv_dnn_ActivationLayerG_new_const_ActivationLayer }
	impl core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] pub fn as_raw_PtrOfActivationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ActivationLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ActivationLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ActivationLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ActivationLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ActivationLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfActivationLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ActivationLayerInt8>` instead, removal in Nov 2024"]
	pub type PtrOfActivationLayerInt8 = core::Ptr<crate::dnn::ActivationLayerInt8>;

	ptr_extern! { crate::dnn::ActivationLayerInt8,
		cv_PtrLcv_dnn_ActivationLayerInt8G_new_null_const, cv_PtrLcv_dnn_ActivationLayerInt8G_delete, cv_PtrLcv_dnn_ActivationLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ActivationLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ActivationLayerInt8, cv_PtrLcv_dnn_ActivationLayerInt8G_new_const_ActivationLayerInt8 }
	impl core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] pub fn as_raw_PtrOfActivationLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfActivationLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ActivationLayerInt8TraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_ActivationLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerInt8Trait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ActivationLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ActivationLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ActivationLayerInt8>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ActivationLayerInt8G_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ActivationLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ActivationLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ActivationLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfActivationLayerInt8")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ArgLayer>` instead, removal in Nov 2024"]
	pub type PtrOfArgLayer = core::Ptr<crate::dnn::ArgLayer>;

	ptr_extern! { crate::dnn::ArgLayer,
		cv_PtrLcv_dnn_ArgLayerG_new_null_const, cv_PtrLcv_dnn_ArgLayerG_delete, cv_PtrLcv_dnn_ArgLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ArgLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ArgLayer, cv_PtrLcv_dnn_ArgLayerG_new_const_ArgLayer }
	impl core::Ptr<crate::dnn::ArgLayer> {
		#[inline] pub fn as_raw_PtrOfArgLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfArgLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ArgLayerTraitConst for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_ArgLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ArgLayerTrait for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_mut_ArgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ArgLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ArgLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ArgLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ArgLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ArgLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ArgLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfArgLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::AsinLayer>` instead, removal in Nov 2024"]
	pub type PtrOfAsinLayer = core::Ptr<crate::dnn::AsinLayer>;

	ptr_extern! { crate::dnn::AsinLayer,
		cv_PtrLcv_dnn_AsinLayerG_new_null_const, cv_PtrLcv_dnn_AsinLayerG_delete, cv_PtrLcv_dnn_AsinLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AsinLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AsinLayer, cv_PtrLcv_dnn_AsinLayerG_new_const_AsinLayer }
	impl core::Ptr<crate::dnn::AsinLayer> {
		#[inline] pub fn as_raw_PtrOfAsinLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAsinLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AsinLayerTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_AsinLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AsinLayerTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_AsinLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AsinLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AsinLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AsinLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AsinLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AsinLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AsinLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AsinLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AsinLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAsinLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::AsinhLayer>` instead, removal in Nov 2024"]
	pub type PtrOfAsinhLayer = core::Ptr<crate::dnn::AsinhLayer>;

	ptr_extern! { crate::dnn::AsinhLayer,
		cv_PtrLcv_dnn_AsinhLayerG_new_null_const, cv_PtrLcv_dnn_AsinhLayerG_delete, cv_PtrLcv_dnn_AsinhLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AsinhLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AsinhLayer, cv_PtrLcv_dnn_AsinhLayerG_new_const_AsinhLayer }
	impl core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] pub fn as_raw_PtrOfAsinhLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAsinhLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AsinhLayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_AsinhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AsinhLayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_AsinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AsinhLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AsinhLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AsinhLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AsinhLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AsinhLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AsinhLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AsinhLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAsinhLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::AtanLayer>` instead, removal in Nov 2024"]
	pub type PtrOfAtanLayer = core::Ptr<crate::dnn::AtanLayer>;

	ptr_extern! { crate::dnn::AtanLayer,
		cv_PtrLcv_dnn_AtanLayerG_new_null_const, cv_PtrLcv_dnn_AtanLayerG_delete, cv_PtrLcv_dnn_AtanLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AtanLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AtanLayer, cv_PtrLcv_dnn_AtanLayerG_new_const_AtanLayer }
	impl core::Ptr<crate::dnn::AtanLayer> {
		#[inline] pub fn as_raw_PtrOfAtanLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAtanLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AtanLayerTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_AtanLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AtanLayerTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_AtanLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AtanLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AtanLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AtanLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AtanLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AtanLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AtanLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AtanLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AtanLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAtanLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::AtanhLayer>` instead, removal in Nov 2024"]
	pub type PtrOfAtanhLayer = core::Ptr<crate::dnn::AtanhLayer>;

	ptr_extern! { crate::dnn::AtanhLayer,
		cv_PtrLcv_dnn_AtanhLayerG_new_null_const, cv_PtrLcv_dnn_AtanhLayerG_delete, cv_PtrLcv_dnn_AtanhLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AtanhLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AtanhLayer, cv_PtrLcv_dnn_AtanhLayerG_new_const_AtanhLayer }
	impl core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] pub fn as_raw_PtrOfAtanhLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAtanhLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AtanhLayerTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_AtanhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AtanhLayerTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_AtanhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AtanhLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AtanhLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AtanhLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_AtanhLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AtanhLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AtanhLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AtanhLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAtanhLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::AttentionLayer>` instead, removal in Nov 2024"]
	pub type PtrOfAttentionLayer = core::Ptr<crate::dnn::AttentionLayer>;

	ptr_extern! { crate::dnn::AttentionLayer,
		cv_PtrLcv_dnn_AttentionLayerG_new_null_const, cv_PtrLcv_dnn_AttentionLayerG_delete, cv_PtrLcv_dnn_AttentionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_AttentionLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::AttentionLayer, cv_PtrLcv_dnn_AttentionLayerG_new_const_AttentionLayer }
	impl core::Ptr<crate::dnn::AttentionLayer> {
		#[inline] pub fn as_raw_PtrOfAttentionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfAttentionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::AttentionLayerTraitConst for core::Ptr<crate::dnn::AttentionLayer> {
		#[inline] fn as_raw_AttentionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::AttentionLayerTrait for core::Ptr<crate::dnn::AttentionLayer> {
		#[inline] fn as_raw_mut_AttentionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::AttentionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::AttentionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AttentionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_AttentionLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::AttentionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::AttentionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::AttentionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_AttentionLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::AttentionLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfAttentionLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::BNLLLayer>` instead, removal in Nov 2024"]
	pub type PtrOfBNLLLayer = core::Ptr<crate::dnn::BNLLLayer>;

	ptr_extern! { crate::dnn::BNLLLayer,
		cv_PtrLcv_dnn_BNLLLayerG_new_null_const, cv_PtrLcv_dnn_BNLLLayerG_delete, cv_PtrLcv_dnn_BNLLLayerG_getInnerPtr_const, cv_PtrLcv_dnn_BNLLLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BNLLLayer, cv_PtrLcv_dnn_BNLLLayerG_new_const_BNLLLayer }
	impl core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] pub fn as_raw_PtrOfBNLLLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBNLLLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BNLLLayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_BNLLLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BNLLLayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BNLLLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BNLLLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BNLLLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_BNLLLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BNLLLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BNLLLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::BNLLLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBNLLLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::BackendNode>` instead, removal in Nov 2024"]
	pub type PtrOfBackendNode = core::Ptr<crate::dnn::BackendNode>;

	ptr_extern! { crate::dnn::BackendNode,
		cv_PtrLcv_dnn_BackendNodeG_new_null_const, cv_PtrLcv_dnn_BackendNodeG_delete, cv_PtrLcv_dnn_BackendNodeG_getInnerPtr_const, cv_PtrLcv_dnn_BackendNodeG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BackendNode, cv_PtrLcv_dnn_BackendNodeG_new_const_BackendNode }
	impl core::Ptr<crate::dnn::BackendNode> {
		#[inline] pub fn as_raw_PtrOfBackendNode(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendNode(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BackendNodeTraitConst for core::Ptr<crate::dnn::BackendNode> {
		#[inline] fn as_raw_BackendNode(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BackendNodeTrait for core::Ptr<crate::dnn::BackendNode> {
		#[inline] fn as_raw_mut_BackendNode(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<crate::dnn::BackendNode> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBackendNode")
				.field("backend_id", &crate::dnn::BackendNodeTraitConst::backend_id(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::BackendWrapper>` instead, removal in Nov 2024"]
	pub type PtrOfBackendWrapper = core::Ptr<crate::dnn::BackendWrapper>;

	ptr_extern! { crate::dnn::BackendWrapper,
		cv_PtrLcv_dnn_BackendWrapperG_new_null_const, cv_PtrLcv_dnn_BackendWrapperG_delete, cv_PtrLcv_dnn_BackendWrapperG_getInnerPtr_const, cv_PtrLcv_dnn_BackendWrapperG_getInnerPtrMut
	}

	impl core::Ptr<crate::dnn::BackendWrapper> {
		#[inline] pub fn as_raw_PtrOfBackendWrapper(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBackendWrapper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BackendWrapperTraitConst for core::Ptr<crate::dnn::BackendWrapper> {
		#[inline] fn as_raw_BackendWrapper(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BackendWrapperTrait for core::Ptr<crate::dnn::BackendWrapper> {
		#[inline] fn as_raw_mut_BackendWrapper(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<crate::dnn::BackendWrapper> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBackendWrapper")
				.field("backend_id", &crate::dnn::BackendWrapperTraitConst::backend_id(self))
				.field("target_id", &crate::dnn::BackendWrapperTraitConst::target_id(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::BaseConvolutionLayer>` instead, removal in Nov 2024"]
	pub type PtrOfBaseConvolutionLayer = core::Ptr<crate::dnn::BaseConvolutionLayer>;

	ptr_extern! { crate::dnn::BaseConvolutionLayer,
		cv_PtrLcv_dnn_BaseConvolutionLayerG_new_null_const, cv_PtrLcv_dnn_BaseConvolutionLayerG_delete, cv_PtrLcv_dnn_BaseConvolutionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_BaseConvolutionLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BaseConvolutionLayer, cv_PtrLcv_dnn_BaseConvolutionLayerG_new_const_BaseConvolutionLayer }
	impl core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBaseConvolutionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BaseConvolutionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BaseConvolutionLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BaseConvolutionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BaseConvolutionLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::BaseConvolutionLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBaseConvolutionLayer")
				.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
				.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
				.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
				.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
				.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
				.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
				.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
				.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
				.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
				.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
				.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::BatchNormLayer>` instead, removal in Nov 2024"]
	pub type PtrOfBatchNormLayer = core::Ptr<crate::dnn::BatchNormLayer>;

	ptr_extern! { crate::dnn::BatchNormLayer,
		cv_PtrLcv_dnn_BatchNormLayerG_new_null_const, cv_PtrLcv_dnn_BatchNormLayerG_delete, cv_PtrLcv_dnn_BatchNormLayerG_getInnerPtr_const, cv_PtrLcv_dnn_BatchNormLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BatchNormLayer, cv_PtrLcv_dnn_BatchNormLayerG_new_const_BatchNormLayer }
	impl core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] pub fn as_raw_PtrOfBatchNormLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BatchNormLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BatchNormLayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BatchNormLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::BatchNormLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBatchNormLayer")
				.field("has_weights", &crate::dnn::BatchNormLayerTraitConst::has_weights(self))
				.field("has_bias", &crate::dnn::BatchNormLayerTraitConst::has_bias(self))
				.field("epsilon", &crate::dnn::BatchNormLayerTraitConst::epsilon(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::BatchNormLayerInt8>` instead, removal in Nov 2024"]
	pub type PtrOfBatchNormLayerInt8 = core::Ptr<crate::dnn::BatchNormLayerInt8>;

	ptr_extern! { crate::dnn::BatchNormLayerInt8,
		cv_PtrLcv_dnn_BatchNormLayerInt8G_new_null_const, cv_PtrLcv_dnn_BatchNormLayerInt8G_delete, cv_PtrLcv_dnn_BatchNormLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_BatchNormLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BatchNormLayerInt8, cv_PtrLcv_dnn_BatchNormLayerInt8G_new_const_BatchNormLayerInt8 }
	impl core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] pub fn as_raw_PtrOfBatchNormLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBatchNormLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BatchNormLayerInt8TraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_BatchNormLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BatchNormLayerInt8Trait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_BatchNormLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfActivationLayer }

	impl crate::dnn::BatchNormLayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_BatchNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BatchNormLayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_BatchNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<crate::dnn::BatchNormLayer>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfBatchNormLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BatchNormLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BatchNormLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::BatchNormLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBatchNormLayerInt8")
				.field("input_sc", &crate::dnn::BatchNormLayerInt8TraitConst::input_sc(self))
				.field("output_sc", &crate::dnn::BatchNormLayerInt8TraitConst::output_sc(self))
				.field("input_zp", &crate::dnn::BatchNormLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::BatchNormLayerInt8TraitConst::output_zp(self))
				.field("has_weights", &crate::dnn::BatchNormLayerTraitConst::has_weights(self))
				.field("has_bias", &crate::dnn::BatchNormLayerTraitConst::has_bias(self))
				.field("epsilon", &crate::dnn::BatchNormLayerTraitConst::epsilon(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::BlankLayer>` instead, removal in Nov 2024"]
	pub type PtrOfBlankLayer = core::Ptr<crate::dnn::BlankLayer>;

	ptr_extern! { crate::dnn::BlankLayer,
		cv_PtrLcv_dnn_BlankLayerG_new_null_const, cv_PtrLcv_dnn_BlankLayerG_delete, cv_PtrLcv_dnn_BlankLayerG_getInnerPtr_const, cv_PtrLcv_dnn_BlankLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::BlankLayer, cv_PtrLcv_dnn_BlankLayerG_new_const_BlankLayer }
	impl core::Ptr<crate::dnn::BlankLayer> {
		#[inline] pub fn as_raw_PtrOfBlankLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfBlankLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::BlankLayerTraitConst for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_BlankLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BlankLayerTrait for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_mut_BlankLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BlankLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_BlankLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::BlankLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::BlankLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_BlankLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::BlankLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfBlankLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::CeilLayer>` instead, removal in Nov 2024"]
	pub type PtrOfCeilLayer = core::Ptr<crate::dnn::CeilLayer>;

	ptr_extern! { crate::dnn::CeilLayer,
		cv_PtrLcv_dnn_CeilLayerG_new_null_const, cv_PtrLcv_dnn_CeilLayerG_delete, cv_PtrLcv_dnn_CeilLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CeilLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CeilLayer, cv_PtrLcv_dnn_CeilLayerG_new_const_CeilLayer }
	impl core::Ptr<crate::dnn::CeilLayer> {
		#[inline] pub fn as_raw_PtrOfCeilLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCeilLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CeilLayerTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_CeilLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CeilLayerTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_CeilLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CeilLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CeilLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CeilLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_CeilLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CeilLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CeilLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CeilLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CeilLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCeilLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::CeluLayer>` instead, removal in Nov 2024"]
	pub type PtrOfCeluLayer = core::Ptr<crate::dnn::CeluLayer>;

	ptr_extern! { crate::dnn::CeluLayer,
		cv_PtrLcv_dnn_CeluLayerG_new_null_const, cv_PtrLcv_dnn_CeluLayerG_delete, cv_PtrLcv_dnn_CeluLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CeluLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CeluLayer, cv_PtrLcv_dnn_CeluLayerG_new_const_CeluLayer }
	impl core::Ptr<crate::dnn::CeluLayer> {
		#[inline] pub fn as_raw_PtrOfCeluLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCeluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CeluLayerTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_CeluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CeluLayerTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_CeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CeluLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CeluLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CeluLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_CeluLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CeluLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CeluLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CeluLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CeluLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCeluLayer")
				.field("alpha", &crate::dnn::CeluLayerTraitConst::alpha(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ChannelsPReLULayer>` instead, removal in Nov 2024"]
	pub type PtrOfChannelsPReLULayer = core::Ptr<crate::dnn::ChannelsPReLULayer>;

	ptr_extern! { crate::dnn::ChannelsPReLULayer,
		cv_PtrLcv_dnn_ChannelsPReLULayerG_new_null_const, cv_PtrLcv_dnn_ChannelsPReLULayerG_delete, cv_PtrLcv_dnn_ChannelsPReLULayerG_getInnerPtr_const, cv_PtrLcv_dnn_ChannelsPReLULayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ChannelsPReLULayer, cv_PtrLcv_dnn_ChannelsPReLULayerG_new_const_ChannelsPReLULayer }
	impl core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] pub fn as_raw_PtrOfChannelsPReLULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfChannelsPReLULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ChannelsPReLULayerTraitConst for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_ChannelsPReLULayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ChannelsPReLULayerTrait for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_mut_ChannelsPReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ChannelsPReLULayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ChannelsPReLULayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ChannelsPReLULayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ChannelsPReLULayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ChannelsPReLULayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ChannelsPReLULayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ChannelsPReLULayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfChannelsPReLULayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::CompareLayer>` instead, removal in Nov 2024"]
	pub type PtrOfCompareLayer = core::Ptr<crate::dnn::CompareLayer>;

	ptr_extern! { crate::dnn::CompareLayer,
		cv_PtrLcv_dnn_CompareLayerG_new_null_const, cv_PtrLcv_dnn_CompareLayerG_delete, cv_PtrLcv_dnn_CompareLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CompareLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CompareLayer, cv_PtrLcv_dnn_CompareLayerG_new_const_CompareLayer }
	impl core::Ptr<crate::dnn::CompareLayer> {
		#[inline] pub fn as_raw_PtrOfCompareLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCompareLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CompareLayerTraitConst for core::Ptr<crate::dnn::CompareLayer> {
		#[inline] fn as_raw_CompareLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CompareLayerTrait for core::Ptr<crate::dnn::CompareLayer> {
		#[inline] fn as_raw_mut_CompareLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CompareLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CompareLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CompareLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CompareLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CompareLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CompareLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CompareLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CompareLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CompareLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCompareLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ConcatLayer>` instead, removal in Nov 2024"]
	pub type PtrOfConcatLayer = core::Ptr<crate::dnn::ConcatLayer>;

	ptr_extern! { crate::dnn::ConcatLayer,
		cv_PtrLcv_dnn_ConcatLayerG_new_null_const, cv_PtrLcv_dnn_ConcatLayerG_delete, cv_PtrLcv_dnn_ConcatLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ConcatLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ConcatLayer, cv_PtrLcv_dnn_ConcatLayerG_new_const_ConcatLayer }
	impl core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] pub fn as_raw_PtrOfConcatLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConcatLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ConcatLayerTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_ConcatLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ConcatLayerTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_ConcatLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConcatLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConcatLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConcatLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConcatLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ConcatLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfConcatLayer")
				.field("axis", &crate::dnn::ConcatLayerTraitConst::axis(self))
				.field("padding", &crate::dnn::ConcatLayerTraitConst::padding(self))
				.field("padding_value", &crate::dnn::ConcatLayerTraitConst::padding_value(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ConstLayer>` instead, removal in Nov 2024"]
	pub type PtrOfConstLayer = core::Ptr<crate::dnn::ConstLayer>;

	ptr_extern! { crate::dnn::ConstLayer,
		cv_PtrLcv_dnn_ConstLayerG_new_null_const, cv_PtrLcv_dnn_ConstLayerG_delete, cv_PtrLcv_dnn_ConstLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ConstLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ConstLayer, cv_PtrLcv_dnn_ConstLayerG_new_const_ConstLayer }
	impl core::Ptr<crate::dnn::ConstLayer> {
		#[inline] pub fn as_raw_PtrOfConstLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConstLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ConstLayerTraitConst for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_ConstLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ConstLayerTrait for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_mut_ConstLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConstLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConstLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConstLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConstLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConstLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ConstLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfConstLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ConvolutionLayer>` instead, removal in Nov 2024"]
	pub type PtrOfConvolutionLayer = core::Ptr<crate::dnn::ConvolutionLayer>;

	ptr_extern! { crate::dnn::ConvolutionLayer,
		cv_PtrLcv_dnn_ConvolutionLayerG_new_null_const, cv_PtrLcv_dnn_ConvolutionLayerG_delete, cv_PtrLcv_dnn_ConvolutionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ConvolutionLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ConvolutionLayer, cv_PtrLcv_dnn_ConvolutionLayerG_new_const_ConvolutionLayer }
	impl core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] pub fn as_raw_PtrOfConvolutionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConvolutionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ConvolutionLayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_ConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ConvolutionLayerTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_mut_ConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayer>, core::Ptr<crate::dnn::BaseConvolutionLayer>, cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfBaseConvolutionLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConvolutionLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ConvolutionLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfConvolutionLayer")
				.field("fused_activation", &crate::dnn::ConvolutionLayerTraitConst::fused_activation(self))
				.field("fused_add", &crate::dnn::ConvolutionLayerTraitConst::fused_add(self))
				.field("use_winograd", &crate::dnn::ConvolutionLayerTraitConst::use_winograd(self))
				.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
				.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
				.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
				.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
				.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
				.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
				.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
				.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
				.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
				.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
				.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ConvolutionLayerInt8>` instead, removal in Nov 2024"]
	pub type PtrOfConvolutionLayerInt8 = core::Ptr<crate::dnn::ConvolutionLayerInt8>;

	ptr_extern! { crate::dnn::ConvolutionLayerInt8,
		cv_PtrLcv_dnn_ConvolutionLayerInt8G_new_null_const, cv_PtrLcv_dnn_ConvolutionLayerInt8G_delete, cv_PtrLcv_dnn_ConvolutionLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ConvolutionLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ConvolutionLayerInt8, cv_PtrLcv_dnn_ConvolutionLayerInt8G_new_const_ConvolutionLayerInt8 }
	impl core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] pub fn as_raw_PtrOfConvolutionLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfConvolutionLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ConvolutionLayerInt8TraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_ConvolutionLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ConvolutionLayerInt8Trait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_mut_ConvolutionLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConvolutionLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayerInt8>, core::Ptr<crate::dnn::BaseConvolutionLayer>, cv_PtrLcv_dnn_ConvolutionLayerInt8G_to_PtrOfBaseConvolutionLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ConvolutionLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConvolutionLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ConvolutionLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfConvolutionLayerInt8")
				.field("input_zp", &crate::dnn::ConvolutionLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::ConvolutionLayerInt8TraitConst::output_zp(self))
				.field("input_sc", &crate::dnn::ConvolutionLayerInt8TraitConst::input_sc(self))
				.field("output_sc", &crate::dnn::ConvolutionLayerInt8TraitConst::output_sc(self))
				.field("per_channel", &crate::dnn::ConvolutionLayerInt8TraitConst::per_channel(self))
				.field("use_winograd", &crate::dnn::ConvolutionLayerInt8TraitConst::use_winograd(self))
				.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
				.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
				.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
				.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
				.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
				.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
				.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
				.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
				.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
				.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
				.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::CorrelationLayer>` instead, removal in Nov 2024"]
	pub type PtrOfCorrelationLayer = core::Ptr<crate::dnn::CorrelationLayer>;

	ptr_extern! { crate::dnn::CorrelationLayer,
		cv_PtrLcv_dnn_CorrelationLayerG_new_null_const, cv_PtrLcv_dnn_CorrelationLayerG_delete, cv_PtrLcv_dnn_CorrelationLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CorrelationLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CorrelationLayer, cv_PtrLcv_dnn_CorrelationLayerG_new_const_CorrelationLayer }
	impl core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] pub fn as_raw_PtrOfCorrelationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCorrelationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CorrelationLayerTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_CorrelationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CorrelationLayerTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_CorrelationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CorrelationLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CorrelationLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CorrelationLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CorrelationLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CorrelationLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCorrelationLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::CosLayer>` instead, removal in Nov 2024"]
	pub type PtrOfCosLayer = core::Ptr<crate::dnn::CosLayer>;

	ptr_extern! { crate::dnn::CosLayer,
		cv_PtrLcv_dnn_CosLayerG_new_null_const, cv_PtrLcv_dnn_CosLayerG_delete, cv_PtrLcv_dnn_CosLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CosLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CosLayer, cv_PtrLcv_dnn_CosLayerG_new_const_CosLayer }
	impl core::Ptr<crate::dnn::CosLayer> {
		#[inline] pub fn as_raw_PtrOfCosLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCosLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CosLayerTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_CosLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CosLayerTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_CosLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CosLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CosLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CosLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_CosLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CosLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CosLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CosLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CosLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCosLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::CoshLayer>` instead, removal in Nov 2024"]
	pub type PtrOfCoshLayer = core::Ptr<crate::dnn::CoshLayer>;

	ptr_extern! { crate::dnn::CoshLayer,
		cv_PtrLcv_dnn_CoshLayerG_new_null_const, cv_PtrLcv_dnn_CoshLayerG_delete, cv_PtrLcv_dnn_CoshLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CoshLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CoshLayer, cv_PtrLcv_dnn_CoshLayerG_new_const_CoshLayer }
	impl core::Ptr<crate::dnn::CoshLayer> {
		#[inline] pub fn as_raw_PtrOfCoshLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCoshLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CoshLayerTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_CoshLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CoshLayerTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_CoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CoshLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CoshLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CoshLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_CoshLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CoshLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CoshLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CoshLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CoshLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCoshLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::CropAndResizeLayer>` instead, removal in Nov 2024"]
	pub type PtrOfCropAndResizeLayer = core::Ptr<crate::dnn::CropAndResizeLayer>;

	ptr_extern! { crate::dnn::CropAndResizeLayer,
		cv_PtrLcv_dnn_CropAndResizeLayerG_new_null_const, cv_PtrLcv_dnn_CropAndResizeLayerG_delete, cv_PtrLcv_dnn_CropAndResizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CropAndResizeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CropAndResizeLayer, cv_PtrLcv_dnn_CropAndResizeLayerG_new_const_CropAndResizeLayer }
	impl core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] pub fn as_raw_PtrOfCropAndResizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCropAndResizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CropAndResizeLayerTraitConst for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_CropAndResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CropAndResizeLayerTrait for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_mut_CropAndResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CropAndResizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CropAndResizeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CropAndResizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CropAndResizeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CropAndResizeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCropAndResizeLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::CropLayer>` instead, removal in Nov 2024"]
	pub type PtrOfCropLayer = core::Ptr<crate::dnn::CropLayer>;

	ptr_extern! { crate::dnn::CropLayer,
		cv_PtrLcv_dnn_CropLayerG_new_null_const, cv_PtrLcv_dnn_CropLayerG_delete, cv_PtrLcv_dnn_CropLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CropLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CropLayer, cv_PtrLcv_dnn_CropLayerG_new_const_CropLayer }
	impl core::Ptr<crate::dnn::CropLayer> {
		#[inline] pub fn as_raw_PtrOfCropLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCropLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CropLayerTraitConst for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_CropLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CropLayerTrait for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_mut_CropLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CropLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CropLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CropLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CropLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CropLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CropLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCropLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::CumSumLayer>` instead, removal in Nov 2024"]
	pub type PtrOfCumSumLayer = core::Ptr<crate::dnn::CumSumLayer>;

	ptr_extern! { crate::dnn::CumSumLayer,
		cv_PtrLcv_dnn_CumSumLayerG_new_null_const, cv_PtrLcv_dnn_CumSumLayerG_delete, cv_PtrLcv_dnn_CumSumLayerG_getInnerPtr_const, cv_PtrLcv_dnn_CumSumLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::CumSumLayer, cv_PtrLcv_dnn_CumSumLayerG_new_const_CumSumLayer }
	impl core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] pub fn as_raw_PtrOfCumSumLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCumSumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::CumSumLayerTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_CumSumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::CumSumLayerTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_CumSumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CumSumLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_CumSumLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::CumSumLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_CumSumLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::CumSumLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCumSumLayer")
				.field("exclusive", &crate::dnn::CumSumLayerTraitConst::exclusive(self))
				.field("reverse", &crate::dnn::CumSumLayerTraitConst::reverse(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::DataAugmentationLayer>` instead, removal in Nov 2024"]
	pub type PtrOfDataAugmentationLayer = core::Ptr<crate::dnn::DataAugmentationLayer>;

	ptr_extern! { crate::dnn::DataAugmentationLayer,
		cv_PtrLcv_dnn_DataAugmentationLayerG_new_null_const, cv_PtrLcv_dnn_DataAugmentationLayerG_delete, cv_PtrLcv_dnn_DataAugmentationLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DataAugmentationLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::DataAugmentationLayer, cv_PtrLcv_dnn_DataAugmentationLayerG_new_const_DataAugmentationLayer }
	impl core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] pub fn as_raw_PtrOfDataAugmentationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDataAugmentationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::DataAugmentationLayerTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_DataAugmentationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DataAugmentationLayerTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_DataAugmentationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DataAugmentationLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DataAugmentationLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DataAugmentationLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DataAugmentationLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::DataAugmentationLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDataAugmentationLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::DeconvolutionLayer>` instead, removal in Nov 2024"]
	pub type PtrOfDeconvolutionLayer = core::Ptr<crate::dnn::DeconvolutionLayer>;

	ptr_extern! { crate::dnn::DeconvolutionLayer,
		cv_PtrLcv_dnn_DeconvolutionLayerG_new_null_const, cv_PtrLcv_dnn_DeconvolutionLayerG_delete, cv_PtrLcv_dnn_DeconvolutionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DeconvolutionLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::DeconvolutionLayer, cv_PtrLcv_dnn_DeconvolutionLayerG_new_const_DeconvolutionLayer }
	impl core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] pub fn as_raw_PtrOfDeconvolutionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDeconvolutionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::DeconvolutionLayerTraitConst for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_DeconvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DeconvolutionLayerTrait for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_mut_DeconvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DeconvolutionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DeconvolutionLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::BaseConvolutionLayerTraitConst for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_BaseConvolutionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::BaseConvolutionLayerTrait for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_mut_BaseConvolutionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DeconvolutionLayer>, core::Ptr<crate::dnn::BaseConvolutionLayer>, cv_PtrLcv_dnn_DeconvolutionLayerG_to_PtrOfBaseConvolutionLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DeconvolutionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DeconvolutionLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::DeconvolutionLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDeconvolutionLayer")
				.field("kernel", &crate::dnn::BaseConvolutionLayerTraitConst::kernel(self))
				.field("stride", &crate::dnn::BaseConvolutionLayerTraitConst::stride(self))
				.field("pad", &crate::dnn::BaseConvolutionLayerTraitConst::pad(self))
				.field("dilation", &crate::dnn::BaseConvolutionLayerTraitConst::dilation(self))
				.field("adjust_pad", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pad(self))
				.field("adjust_pads", &crate::dnn::BaseConvolutionLayerTraitConst::adjust_pads(self))
				.field("kernel_size", &crate::dnn::BaseConvolutionLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::BaseConvolutionLayerTraitConst::strides(self))
				.field("dilations", &crate::dnn::BaseConvolutionLayerTraitConst::dilations(self))
				.field("pads_begin", &crate::dnn::BaseConvolutionLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::BaseConvolutionLayerTraitConst::pads_end(self))
				.field("pad_mode", &crate::dnn::BaseConvolutionLayerTraitConst::pad_mode(self))
				.field("num_output", &crate::dnn::BaseConvolutionLayerTraitConst::num_output(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::DepthToSpaceLayer>` instead, removal in Nov 2024"]
	pub type PtrOfDepthToSpaceLayer = core::Ptr<crate::dnn::DepthToSpaceLayer>;

	ptr_extern! { crate::dnn::DepthToSpaceLayer,
		cv_PtrLcv_dnn_DepthToSpaceLayerG_new_null_const, cv_PtrLcv_dnn_DepthToSpaceLayerG_delete, cv_PtrLcv_dnn_DepthToSpaceLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DepthToSpaceLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::DepthToSpaceLayer, cv_PtrLcv_dnn_DepthToSpaceLayerG_new_const_DepthToSpaceLayer }
	impl core::Ptr<crate::dnn::DepthToSpaceLayer> {
		#[inline] pub fn as_raw_PtrOfDepthToSpaceLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDepthToSpaceLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::DepthToSpaceLayerTraitConst for core::Ptr<crate::dnn::DepthToSpaceLayer> {
		#[inline] fn as_raw_DepthToSpaceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DepthToSpaceLayerTrait for core::Ptr<crate::dnn::DepthToSpaceLayer> {
		#[inline] fn as_raw_mut_DepthToSpaceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DepthToSpaceLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DepthToSpaceLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DepthToSpaceLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DepthToSpaceLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DepthToSpaceLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DepthToSpaceLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DepthToSpaceLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DepthToSpaceLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::DepthToSpaceLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDepthToSpaceLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::DequantizeDynamicLayer>` instead, removal in Nov 2024"]
	pub type PtrOfDequantizeDynamicLayer = core::Ptr<crate::dnn::DequantizeDynamicLayer>;

	ptr_extern! { crate::dnn::DequantizeDynamicLayer,
		cv_PtrLcv_dnn_DequantizeDynamicLayerG_new_null_const, cv_PtrLcv_dnn_DequantizeDynamicLayerG_delete, cv_PtrLcv_dnn_DequantizeDynamicLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DequantizeDynamicLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::DequantizeDynamicLayer, cv_PtrLcv_dnn_DequantizeDynamicLayerG_new_const_DequantizeDynamicLayer }
	impl core::Ptr<crate::dnn::DequantizeDynamicLayer> {
		#[inline] pub fn as_raw_PtrOfDequantizeDynamicLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDequantizeDynamicLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::DequantizeDynamicLayerTraitConst for core::Ptr<crate::dnn::DequantizeDynamicLayer> {
		#[inline] fn as_raw_DequantizeDynamicLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DequantizeDynamicLayerTrait for core::Ptr<crate::dnn::DequantizeDynamicLayer> {
		#[inline] fn as_raw_mut_DequantizeDynamicLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DequantizeDynamicLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DequantizeDynamicLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DequantizeDynamicLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DequantizeDynamicLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::DequantizeLayerTraitConst for core::Ptr<crate::dnn::DequantizeDynamicLayer> {
		#[inline] fn as_raw_DequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DequantizeLayerTrait for core::Ptr<crate::dnn::DequantizeDynamicLayer> {
		#[inline] fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DequantizeDynamicLayer>, core::Ptr<crate::dnn::DequantizeLayer>, cv_PtrLcv_dnn_DequantizeDynamicLayerG_to_PtrOfDequantizeLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DequantizeDynamicLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DequantizeDynamicLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DequantizeDynamicLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DequantizeDynamicLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::DequantizeDynamicLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDequantizeDynamicLayer")
				.field("scales", &crate::dnn::DequantizeLayerTraitConst::scales(self))
				.field("zeropoints", &crate::dnn::DequantizeLayerTraitConst::zeropoints(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::DequantizeLayer>` instead, removal in Nov 2024"]
	pub type PtrOfDequantizeLayer = core::Ptr<crate::dnn::DequantizeLayer>;

	ptr_extern! { crate::dnn::DequantizeLayer,
		cv_PtrLcv_dnn_DequantizeLayerG_new_null_const, cv_PtrLcv_dnn_DequantizeLayerG_delete, cv_PtrLcv_dnn_DequantizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DequantizeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::DequantizeLayer, cv_PtrLcv_dnn_DequantizeLayerG_new_const_DequantizeLayer }
	impl core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] pub fn as_raw_PtrOfDequantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDequantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::DequantizeLayerTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_DequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DequantizeLayerTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_DequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DequantizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DequantizeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DequantizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DequantizeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::DequantizeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDequantizeLayer")
				.field("scales", &crate::dnn::DequantizeLayerTraitConst::scales(self))
				.field("zeropoints", &crate::dnn::DequantizeLayerTraitConst::zeropoints(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::DetectionOutputLayer>` instead, removal in Nov 2024"]
	pub type PtrOfDetectionOutputLayer = core::Ptr<crate::dnn::DetectionOutputLayer>;

	ptr_extern! { crate::dnn::DetectionOutputLayer,
		cv_PtrLcv_dnn_DetectionOutputLayerG_new_null_const, cv_PtrLcv_dnn_DetectionOutputLayerG_delete, cv_PtrLcv_dnn_DetectionOutputLayerG_getInnerPtr_const, cv_PtrLcv_dnn_DetectionOutputLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::DetectionOutputLayer, cv_PtrLcv_dnn_DetectionOutputLayerG_new_const_DetectionOutputLayer }
	impl core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] pub fn as_raw_PtrOfDetectionOutputLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfDetectionOutputLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::DetectionOutputLayerTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_DetectionOutputLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::DetectionOutputLayerTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_DetectionOutputLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DetectionOutputLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_DetectionOutputLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::DetectionOutputLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_DetectionOutputLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::DetectionOutputLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfDetectionOutputLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ELULayer>` instead, removal in Nov 2024"]
	pub type PtrOfELULayer = core::Ptr<crate::dnn::ELULayer>;

	ptr_extern! { crate::dnn::ELULayer,
		cv_PtrLcv_dnn_ELULayerG_new_null_const, cv_PtrLcv_dnn_ELULayerG_delete, cv_PtrLcv_dnn_ELULayerG_getInnerPtr_const, cv_PtrLcv_dnn_ELULayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ELULayer, cv_PtrLcv_dnn_ELULayerG_new_const_ELULayer }
	impl core::Ptr<crate::dnn::ELULayer> {
		#[inline] pub fn as_raw_PtrOfELULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfELULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ELULayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_ELULayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ELULayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_ELULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ELULayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ELULayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ELULayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ELULayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ELULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ELULayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ELULayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ELULayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfELULayer")
				.field("alpha", &crate::dnn::ELULayerTraitConst::alpha(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::EinsumLayer>` instead, removal in Nov 2024"]
	pub type PtrOfEinsumLayer = core::Ptr<crate::dnn::EinsumLayer>;

	ptr_extern! { crate::dnn::EinsumLayer,
		cv_PtrLcv_dnn_EinsumLayerG_new_null_const, cv_PtrLcv_dnn_EinsumLayerG_delete, cv_PtrLcv_dnn_EinsumLayerG_getInnerPtr_const, cv_PtrLcv_dnn_EinsumLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::EinsumLayer, cv_PtrLcv_dnn_EinsumLayerG_new_const_EinsumLayer }
	impl core::Ptr<crate::dnn::EinsumLayer> {
		#[inline] pub fn as_raw_PtrOfEinsumLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEinsumLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::EinsumLayerTraitConst for core::Ptr<crate::dnn::EinsumLayer> {
		#[inline] fn as_raw_EinsumLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::EinsumLayerTrait for core::Ptr<crate::dnn::EinsumLayer> {
		#[inline] fn as_raw_mut_EinsumLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::EinsumLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::EinsumLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::EinsumLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_EinsumLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::EinsumLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::EinsumLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::EinsumLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_EinsumLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::EinsumLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfEinsumLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::EltwiseLayer>` instead, removal in Nov 2024"]
	pub type PtrOfEltwiseLayer = core::Ptr<crate::dnn::EltwiseLayer>;

	ptr_extern! { crate::dnn::EltwiseLayer,
		cv_PtrLcv_dnn_EltwiseLayerG_new_null_const, cv_PtrLcv_dnn_EltwiseLayerG_delete, cv_PtrLcv_dnn_EltwiseLayerG_getInnerPtr_const, cv_PtrLcv_dnn_EltwiseLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::EltwiseLayer, cv_PtrLcv_dnn_EltwiseLayerG_new_const_EltwiseLayer }
	impl core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] pub fn as_raw_PtrOfEltwiseLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::EltwiseLayerTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_EltwiseLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::EltwiseLayerTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_EltwiseLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::EltwiseLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_EltwiseLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::EltwiseLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_EltwiseLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::EltwiseLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfEltwiseLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::EltwiseLayerInt8>` instead, removal in Nov 2024"]
	pub type PtrOfEltwiseLayerInt8 = core::Ptr<crate::dnn::EltwiseLayerInt8>;

	ptr_extern! { crate::dnn::EltwiseLayerInt8,
		cv_PtrLcv_dnn_EltwiseLayerInt8G_new_null_const, cv_PtrLcv_dnn_EltwiseLayerInt8G_delete, cv_PtrLcv_dnn_EltwiseLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_EltwiseLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::EltwiseLayerInt8, cv_PtrLcv_dnn_EltwiseLayerInt8G_new_const_EltwiseLayerInt8 }
	impl core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] pub fn as_raw_PtrOfEltwiseLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfEltwiseLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::EltwiseLayerInt8TraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_EltwiseLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::EltwiseLayerInt8Trait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_EltwiseLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::EltwiseLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_EltwiseLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::EltwiseLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_EltwiseLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::EltwiseLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfEltwiseLayerInt8")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ErfLayer>` instead, removal in Nov 2024"]
	pub type PtrOfErfLayer = core::Ptr<crate::dnn::ErfLayer>;

	ptr_extern! { crate::dnn::ErfLayer,
		cv_PtrLcv_dnn_ErfLayerG_new_null_const, cv_PtrLcv_dnn_ErfLayerG_delete, cv_PtrLcv_dnn_ErfLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ErfLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ErfLayer, cv_PtrLcv_dnn_ErfLayerG_new_const_ErfLayer }
	impl core::Ptr<crate::dnn::ErfLayer> {
		#[inline] pub fn as_raw_PtrOfErfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfErfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ErfLayerTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_ErfLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ErfLayerTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_ErfLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ErfLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ErfLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ErfLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ErfLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ErfLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ErfLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ErfLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ErfLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfErfLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ExpLayer>` instead, removal in Nov 2024"]
	pub type PtrOfExpLayer = core::Ptr<crate::dnn::ExpLayer>;

	ptr_extern! { crate::dnn::ExpLayer,
		cv_PtrLcv_dnn_ExpLayerG_new_null_const, cv_PtrLcv_dnn_ExpLayerG_delete, cv_PtrLcv_dnn_ExpLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ExpLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ExpLayer, cv_PtrLcv_dnn_ExpLayerG_new_const_ExpLayer }
	impl core::Ptr<crate::dnn::ExpLayer> {
		#[inline] pub fn as_raw_PtrOfExpLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfExpLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ExpLayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_ExpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ExpLayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ExpLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ExpLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ExpLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ExpLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ExpLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ExpLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ExpLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ExpLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfExpLayer")
				.field("base", &crate::dnn::ExpLayerTraitConst::base(self))
				.field("scale", &crate::dnn::ExpLayerTraitConst::scale(self))
				.field("shift", &crate::dnn::ExpLayerTraitConst::shift(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ExpandLayer>` instead, removal in Nov 2024"]
	pub type PtrOfExpandLayer = core::Ptr<crate::dnn::ExpandLayer>;

	ptr_extern! { crate::dnn::ExpandLayer,
		cv_PtrLcv_dnn_ExpandLayerG_new_null_const, cv_PtrLcv_dnn_ExpandLayerG_delete, cv_PtrLcv_dnn_ExpandLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ExpandLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ExpandLayer, cv_PtrLcv_dnn_ExpandLayerG_new_const_ExpandLayer }
	impl core::Ptr<crate::dnn::ExpandLayer> {
		#[inline] pub fn as_raw_PtrOfExpandLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfExpandLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ExpandLayerTraitConst for core::Ptr<crate::dnn::ExpandLayer> {
		#[inline] fn as_raw_ExpandLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ExpandLayerTrait for core::Ptr<crate::dnn::ExpandLayer> {
		#[inline] fn as_raw_mut_ExpandLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ExpandLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ExpandLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ExpandLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ExpandLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ExpandLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ExpandLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ExpandLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ExpandLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ExpandLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfExpandLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::FlattenLayer>` instead, removal in Nov 2024"]
	pub type PtrOfFlattenLayer = core::Ptr<crate::dnn::FlattenLayer>;

	ptr_extern! { crate::dnn::FlattenLayer,
		cv_PtrLcv_dnn_FlattenLayerG_new_null_const, cv_PtrLcv_dnn_FlattenLayerG_delete, cv_PtrLcv_dnn_FlattenLayerG_getInnerPtr_const, cv_PtrLcv_dnn_FlattenLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::FlattenLayer, cv_PtrLcv_dnn_FlattenLayerG_new_const_FlattenLayer }
	impl core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] pub fn as_raw_PtrOfFlattenLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlattenLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::FlattenLayerTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_FlattenLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::FlattenLayerTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_FlattenLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FlattenLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_FlattenLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FlattenLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_FlattenLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::FlattenLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFlattenLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::FloorLayer>` instead, removal in Nov 2024"]
	pub type PtrOfFloorLayer = core::Ptr<crate::dnn::FloorLayer>;

	ptr_extern! { crate::dnn::FloorLayer,
		cv_PtrLcv_dnn_FloorLayerG_new_null_const, cv_PtrLcv_dnn_FloorLayerG_delete, cv_PtrLcv_dnn_FloorLayerG_getInnerPtr_const, cv_PtrLcv_dnn_FloorLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::FloorLayer, cv_PtrLcv_dnn_FloorLayerG_new_const_FloorLayer }
	impl core::Ptr<crate::dnn::FloorLayer> {
		#[inline] pub fn as_raw_PtrOfFloorLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFloorLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::FloorLayerTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_FloorLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::FloorLayerTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_FloorLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FloorLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_FloorLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FloorLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_FloorLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FloorLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FloorLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_FloorLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::FloorLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFloorLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::FlowWarpLayer>` instead, removal in Nov 2024"]
	pub type PtrOfFlowWarpLayer = core::Ptr<crate::dnn::FlowWarpLayer>;

	ptr_extern! { crate::dnn::FlowWarpLayer,
		cv_PtrLcv_dnn_FlowWarpLayerG_new_null_const, cv_PtrLcv_dnn_FlowWarpLayerG_delete, cv_PtrLcv_dnn_FlowWarpLayerG_getInnerPtr_const, cv_PtrLcv_dnn_FlowWarpLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::FlowWarpLayer, cv_PtrLcv_dnn_FlowWarpLayerG_new_const_FlowWarpLayer }
	impl core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] pub fn as_raw_PtrOfFlowWarpLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfFlowWarpLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::FlowWarpLayerTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_FlowWarpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::FlowWarpLayerTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_FlowWarpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FlowWarpLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_FlowWarpLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::FlowWarpLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_FlowWarpLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::FlowWarpLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfFlowWarpLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::GRULayer>` instead, removal in Nov 2024"]
	pub type PtrOfGRULayer = core::Ptr<crate::dnn::GRULayer>;

	ptr_extern! { crate::dnn::GRULayer,
		cv_PtrLcv_dnn_GRULayerG_new_null_const, cv_PtrLcv_dnn_GRULayerG_delete, cv_PtrLcv_dnn_GRULayerG_getInnerPtr_const, cv_PtrLcv_dnn_GRULayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::GRULayer, cv_PtrLcv_dnn_GRULayerG_new_const_GRULayer }
	impl core::Ptr<crate::dnn::GRULayer> {
		#[inline] pub fn as_raw_PtrOfGRULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGRULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::GRULayerTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_GRULayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::GRULayerTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_GRULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GRULayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GRULayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GRULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GRULayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GRULayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::GRULayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGRULayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::GatherElementsLayer>` instead, removal in Nov 2024"]
	pub type PtrOfGatherElementsLayer = core::Ptr<crate::dnn::GatherElementsLayer>;

	ptr_extern! { crate::dnn::GatherElementsLayer,
		cv_PtrLcv_dnn_GatherElementsLayerG_new_null_const, cv_PtrLcv_dnn_GatherElementsLayerG_delete, cv_PtrLcv_dnn_GatherElementsLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GatherElementsLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::GatherElementsLayer, cv_PtrLcv_dnn_GatherElementsLayerG_new_const_GatherElementsLayer }
	impl core::Ptr<crate::dnn::GatherElementsLayer> {
		#[inline] pub fn as_raw_PtrOfGatherElementsLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGatherElementsLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::GatherElementsLayerTraitConst for core::Ptr<crate::dnn::GatherElementsLayer> {
		#[inline] fn as_raw_GatherElementsLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::GatherElementsLayerTrait for core::Ptr<crate::dnn::GatherElementsLayer> {
		#[inline] fn as_raw_mut_GatherElementsLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GatherElementsLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GatherElementsLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GatherElementsLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GatherElementsLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GatherElementsLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GatherElementsLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GatherElementsLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GatherElementsLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::GatherElementsLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGatherElementsLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::GatherLayer>` instead, removal in Nov 2024"]
	pub type PtrOfGatherLayer = core::Ptr<crate::dnn::GatherLayer>;

	ptr_extern! { crate::dnn::GatherLayer,
		cv_PtrLcv_dnn_GatherLayerG_new_null_const, cv_PtrLcv_dnn_GatherLayerG_delete, cv_PtrLcv_dnn_GatherLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GatherLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::GatherLayer, cv_PtrLcv_dnn_GatherLayerG_new_const_GatherLayer }
	impl core::Ptr<crate::dnn::GatherLayer> {
		#[inline] pub fn as_raw_PtrOfGatherLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGatherLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::GatherLayerTraitConst for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_GatherLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::GatherLayerTrait for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_mut_GatherLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GatherLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GatherLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GatherLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GatherLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GatherLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::GatherLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGatherLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::GeluApproximationLayer>` instead, removal in Nov 2024"]
	pub type PtrOfGeluApproximationLayer = core::Ptr<crate::dnn::GeluApproximationLayer>;

	ptr_extern! { crate::dnn::GeluApproximationLayer,
		cv_PtrLcv_dnn_GeluApproximationLayerG_new_null_const, cv_PtrLcv_dnn_GeluApproximationLayerG_delete, cv_PtrLcv_dnn_GeluApproximationLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GeluApproximationLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::GeluApproximationLayer, cv_PtrLcv_dnn_GeluApproximationLayerG_new_const_GeluApproximationLayer }
	impl core::Ptr<crate::dnn::GeluApproximationLayer> {
		#[inline] pub fn as_raw_PtrOfGeluApproximationLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeluApproximationLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::GeluApproximationLayerTraitConst for core::Ptr<crate::dnn::GeluApproximationLayer> {
		#[inline] fn as_raw_GeluApproximationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::GeluApproximationLayerTrait for core::Ptr<crate::dnn::GeluApproximationLayer> {
		#[inline] fn as_raw_mut_GeluApproximationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GeluApproximationLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GeluApproximationLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GeluApproximationLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GeluApproximationLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::GeluApproximationLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::GeluApproximationLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GeluApproximationLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_GeluApproximationLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GeluApproximationLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GeluApproximationLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GeluApproximationLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GeluApproximationLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::GeluApproximationLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGeluApproximationLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::GeluLayer>` instead, removal in Nov 2024"]
	pub type PtrOfGeluLayer = core::Ptr<crate::dnn::GeluLayer>;

	ptr_extern! { crate::dnn::GeluLayer,
		cv_PtrLcv_dnn_GeluLayerG_new_null_const, cv_PtrLcv_dnn_GeluLayerG_delete, cv_PtrLcv_dnn_GeluLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GeluLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::GeluLayer, cv_PtrLcv_dnn_GeluLayerG_new_const_GeluLayer }
	impl core::Ptr<crate::dnn::GeluLayer> {
		#[inline] pub fn as_raw_PtrOfGeluLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::GeluLayerTraitConst for core::Ptr<crate::dnn::GeluLayer> {
		#[inline] fn as_raw_GeluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::GeluLayerTrait for core::Ptr<crate::dnn::GeluLayer> {
		#[inline] fn as_raw_mut_GeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GeluLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GeluLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GeluLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GeluLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::GeluLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::GeluLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GeluLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_GeluLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GeluLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GeluLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GeluLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GeluLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::GeluLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGeluLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::GemmLayer>` instead, removal in Nov 2024"]
	pub type PtrOfGemmLayer = core::Ptr<crate::dnn::GemmLayer>;

	ptr_extern! { crate::dnn::GemmLayer,
		cv_PtrLcv_dnn_GemmLayerG_new_null_const, cv_PtrLcv_dnn_GemmLayerG_delete, cv_PtrLcv_dnn_GemmLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GemmLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::GemmLayer, cv_PtrLcv_dnn_GemmLayerG_new_const_GemmLayer }
	impl core::Ptr<crate::dnn::GemmLayer> {
		#[inline] pub fn as_raw_PtrOfGemmLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGemmLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::GemmLayerTraitConst for core::Ptr<crate::dnn::GemmLayer> {
		#[inline] fn as_raw_GemmLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::GemmLayerTrait for core::Ptr<crate::dnn::GemmLayer> {
		#[inline] fn as_raw_mut_GemmLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GemmLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GemmLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GemmLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GemmLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GemmLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GemmLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GemmLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GemmLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::GemmLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGemmLayer")
				.field("trans_a", &crate::dnn::GemmLayerTraitConst::trans_a(self))
				.field("trans_b", &crate::dnn::GemmLayerTraitConst::trans_b(self))
				.field("alpha", &crate::dnn::GemmLayerTraitConst::alpha(self))
				.field("beta", &crate::dnn::GemmLayerTraitConst::beta(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::GroupNormLayer>` instead, removal in Nov 2024"]
	pub type PtrOfGroupNormLayer = core::Ptr<crate::dnn::GroupNormLayer>;

	ptr_extern! { crate::dnn::GroupNormLayer,
		cv_PtrLcv_dnn_GroupNormLayerG_new_null_const, cv_PtrLcv_dnn_GroupNormLayerG_delete, cv_PtrLcv_dnn_GroupNormLayerG_getInnerPtr_const, cv_PtrLcv_dnn_GroupNormLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::GroupNormLayer, cv_PtrLcv_dnn_GroupNormLayerG_new_const_GroupNormLayer }
	impl core::Ptr<crate::dnn::GroupNormLayer> {
		#[inline] pub fn as_raw_PtrOfGroupNormLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGroupNormLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::GroupNormLayerTraitConst for core::Ptr<crate::dnn::GroupNormLayer> {
		#[inline] fn as_raw_GroupNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::GroupNormLayerTrait for core::Ptr<crate::dnn::GroupNormLayer> {
		#[inline] fn as_raw_mut_GroupNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::GroupNormLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::GroupNormLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GroupNormLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_GroupNormLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::GroupNormLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::GroupNormLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::GroupNormLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_GroupNormLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::GroupNormLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGroupNormLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::HardSigmoidLayer>` instead, removal in Nov 2024"]
	pub type PtrOfHardSigmoidLayer = core::Ptr<crate::dnn::HardSigmoidLayer>;

	ptr_extern! { crate::dnn::HardSigmoidLayer,
		cv_PtrLcv_dnn_HardSigmoidLayerG_new_null_const, cv_PtrLcv_dnn_HardSigmoidLayerG_delete, cv_PtrLcv_dnn_HardSigmoidLayerG_getInnerPtr_const, cv_PtrLcv_dnn_HardSigmoidLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::HardSigmoidLayer, cv_PtrLcv_dnn_HardSigmoidLayerG_new_const_HardSigmoidLayer }
	impl core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] pub fn as_raw_PtrOfHardSigmoidLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHardSigmoidLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::HardSigmoidLayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_HardSigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::HardSigmoidLayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_HardSigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::HardSigmoidLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_HardSigmoidLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::HardSigmoidLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_HardSigmoidLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::HardSigmoidLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_HardSigmoidLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::HardSigmoidLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfHardSigmoidLayer")
				.field("alpha", &crate::dnn::HardSigmoidLayerTraitConst::alpha(self))
				.field("beta", &crate::dnn::HardSigmoidLayerTraitConst::beta(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::HardSwishLayer>` instead, removal in Nov 2024"]
	pub type PtrOfHardSwishLayer = core::Ptr<crate::dnn::HardSwishLayer>;

	ptr_extern! { crate::dnn::HardSwishLayer,
		cv_PtrLcv_dnn_HardSwishLayerG_new_null_const, cv_PtrLcv_dnn_HardSwishLayerG_delete, cv_PtrLcv_dnn_HardSwishLayerG_getInnerPtr_const, cv_PtrLcv_dnn_HardSwishLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::HardSwishLayer, cv_PtrLcv_dnn_HardSwishLayerG_new_const_HardSwishLayer }
	impl core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] pub fn as_raw_PtrOfHardSwishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfHardSwishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::HardSwishLayerTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_HardSwishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::HardSwishLayerTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_HardSwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::HardSwishLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_HardSwishLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::HardSwishLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_HardSwishLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::HardSwishLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_HardSwishLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::HardSwishLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfHardSwishLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::InnerProductLayer>` instead, removal in Nov 2024"]
	pub type PtrOfInnerProductLayer = core::Ptr<crate::dnn::InnerProductLayer>;

	ptr_extern! { crate::dnn::InnerProductLayer,
		cv_PtrLcv_dnn_InnerProductLayerG_new_null_const, cv_PtrLcv_dnn_InnerProductLayerG_delete, cv_PtrLcv_dnn_InnerProductLayerG_getInnerPtr_const, cv_PtrLcv_dnn_InnerProductLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::InnerProductLayer, cv_PtrLcv_dnn_InnerProductLayerG_new_const_InnerProductLayer }
	impl core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] pub fn as_raw_PtrOfInnerProductLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::InnerProductLayerTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::InnerProductLayerTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_InnerProductLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_InnerProductLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::InnerProductLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfInnerProductLayer")
				.field("axis", &crate::dnn::InnerProductLayerTraitConst::axis(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::InnerProductLayerInt8>` instead, removal in Nov 2024"]
	pub type PtrOfInnerProductLayerInt8 = core::Ptr<crate::dnn::InnerProductLayerInt8>;

	ptr_extern! { crate::dnn::InnerProductLayerInt8,
		cv_PtrLcv_dnn_InnerProductLayerInt8G_new_null_const, cv_PtrLcv_dnn_InnerProductLayerInt8G_delete, cv_PtrLcv_dnn_InnerProductLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_InnerProductLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::InnerProductLayerInt8, cv_PtrLcv_dnn_InnerProductLayerInt8G_new_const_InnerProductLayerInt8 }
	impl core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] pub fn as_raw_PtrOfInnerProductLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInnerProductLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::InnerProductLayerInt8TraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_InnerProductLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::InnerProductLayerInt8Trait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_InnerProductLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::InnerProductLayerTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_InnerProductLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::InnerProductLayerTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_InnerProductLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayerInt8>, core::Ptr<crate::dnn::InnerProductLayer>, cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfInnerProductLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InnerProductLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_InnerProductLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::InnerProductLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfInnerProductLayerInt8")
				.field("input_zp", &crate::dnn::InnerProductLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::InnerProductLayerInt8TraitConst::output_zp(self))
				.field("input_sc", &crate::dnn::InnerProductLayerInt8TraitConst::input_sc(self))
				.field("output_sc", &crate::dnn::InnerProductLayerInt8TraitConst::output_sc(self))
				.field("per_channel", &crate::dnn::InnerProductLayerInt8TraitConst::per_channel(self))
				.field("axis", &crate::dnn::InnerProductLayerTraitConst::axis(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::InstanceNormLayer>` instead, removal in Nov 2024"]
	pub type PtrOfInstanceNormLayer = core::Ptr<crate::dnn::InstanceNormLayer>;

	ptr_extern! { crate::dnn::InstanceNormLayer,
		cv_PtrLcv_dnn_InstanceNormLayerG_new_null_const, cv_PtrLcv_dnn_InstanceNormLayerG_delete, cv_PtrLcv_dnn_InstanceNormLayerG_getInnerPtr_const, cv_PtrLcv_dnn_InstanceNormLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::InstanceNormLayer, cv_PtrLcv_dnn_InstanceNormLayerG_new_const_InstanceNormLayer }
	impl core::Ptr<crate::dnn::InstanceNormLayer> {
		#[inline] pub fn as_raw_PtrOfInstanceNormLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInstanceNormLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::InstanceNormLayerTraitConst for core::Ptr<crate::dnn::InstanceNormLayer> {
		#[inline] fn as_raw_InstanceNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::InstanceNormLayerTrait for core::Ptr<crate::dnn::InstanceNormLayer> {
		#[inline] fn as_raw_mut_InstanceNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InstanceNormLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InstanceNormLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InstanceNormLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_InstanceNormLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InstanceNormLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InstanceNormLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InstanceNormLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_InstanceNormLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::InstanceNormLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfInstanceNormLayer")
				.field("epsilon", &crate::dnn::InstanceNormLayerTraitConst::epsilon(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::InterpLayer>` instead, removal in Nov 2024"]
	pub type PtrOfInterpLayer = core::Ptr<crate::dnn::InterpLayer>;

	ptr_extern! { crate::dnn::InterpLayer,
		cv_PtrLcv_dnn_InterpLayerG_new_null_const, cv_PtrLcv_dnn_InterpLayerG_delete, cv_PtrLcv_dnn_InterpLayerG_getInnerPtr_const, cv_PtrLcv_dnn_InterpLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::InterpLayer, cv_PtrLcv_dnn_InterpLayerG_new_const_InterpLayer }
	impl core::Ptr<crate::dnn::InterpLayer> {
		#[inline] pub fn as_raw_PtrOfInterpLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfInterpLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::InterpLayerTraitConst for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_InterpLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::InterpLayerTrait for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_mut_InterpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InterpLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_InterpLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::InterpLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::InterpLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_InterpLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::InterpLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfInterpLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::LRNLayer>` instead, removal in Nov 2024"]
	pub type PtrOfLRNLayer = core::Ptr<crate::dnn::LRNLayer>;

	ptr_extern! { crate::dnn::LRNLayer,
		cv_PtrLcv_dnn_LRNLayerG_new_null_const, cv_PtrLcv_dnn_LRNLayerG_delete, cv_PtrLcv_dnn_LRNLayerG_getInnerPtr_const, cv_PtrLcv_dnn_LRNLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::LRNLayer, cv_PtrLcv_dnn_LRNLayerG_new_const_LRNLayer }
	impl core::Ptr<crate::dnn::LRNLayer> {
		#[inline] pub fn as_raw_PtrOfLRNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLRNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::LRNLayerTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_LRNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LRNLayerTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_LRNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LRNLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LRNLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LRNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LRNLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_LRNLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::LRNLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfLRNLayer")
				.field("typ", &crate::dnn::LRNLayerTraitConst::typ(self))
				.field("size", &crate::dnn::LRNLayerTraitConst::size(self))
				.field("alpha", &crate::dnn::LRNLayerTraitConst::alpha(self))
				.field("beta", &crate::dnn::LRNLayerTraitConst::beta(self))
				.field("bias", &crate::dnn::LRNLayerTraitConst::bias(self))
				.field("norm_by_size", &crate::dnn::LRNLayerTraitConst::norm_by_size(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::LSTMLayer>` instead, removal in Nov 2024"]
	pub type PtrOfLSTMLayer = core::Ptr<crate::dnn::LSTMLayer>;

	ptr_extern! { crate::dnn::LSTMLayer,
		cv_PtrLcv_dnn_LSTMLayerG_new_null_const, cv_PtrLcv_dnn_LSTMLayerG_delete, cv_PtrLcv_dnn_LSTMLayerG_getInnerPtr_const, cv_PtrLcv_dnn_LSTMLayerG_getInnerPtrMut
	}

	impl core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] pub fn as_raw_PtrOfLSTMLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLSTMLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::LSTMLayerTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_LSTMLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LSTMLayerTrait for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_LSTMLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LSTMLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LSTMLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LSTMLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_LSTMLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::LSTMLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfLSTMLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::Layer>` instead, removal in Nov 2024"]
	pub type PtrOfLayer = core::Ptr<crate::dnn::Layer>;

	ptr_extern! { crate::dnn::Layer,
		cv_PtrLcv_dnn_LayerG_new_null_const, cv_PtrLcv_dnn_LayerG_delete, cv_PtrLcv_dnn_LayerG_getInnerPtr_const, cv_PtrLcv_dnn_LayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::Layer, cv_PtrLcv_dnn_LayerG_new_const_Layer }
	impl core::Ptr<crate::dnn::Layer> {
		#[inline] pub fn as_raw_PtrOfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::Layer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::Layer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LayerG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<crate::dnn::Layer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::LayerNormLayer>` instead, removal in Nov 2024"]
	pub type PtrOfLayerNormLayer = core::Ptr<crate::dnn::LayerNormLayer>;

	ptr_extern! { crate::dnn::LayerNormLayer,
		cv_PtrLcv_dnn_LayerNormLayerG_new_null_const, cv_PtrLcv_dnn_LayerNormLayerG_delete, cv_PtrLcv_dnn_LayerNormLayerG_getInnerPtr_const, cv_PtrLcv_dnn_LayerNormLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::LayerNormLayer, cv_PtrLcv_dnn_LayerNormLayerG_new_const_LayerNormLayer }
	impl core::Ptr<crate::dnn::LayerNormLayer> {
		#[inline] pub fn as_raw_PtrOfLayerNormLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLayerNormLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::LayerNormLayerTraitConst for core::Ptr<crate::dnn::LayerNormLayer> {
		#[inline] fn as_raw_LayerNormLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerNormLayerTrait for core::Ptr<crate::dnn::LayerNormLayer> {
		#[inline] fn as_raw_mut_LayerNormLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LayerNormLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LayerNormLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LayerNormLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LayerNormLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LayerNormLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LayerNormLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LayerNormLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_LayerNormLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::LayerNormLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfLayerNormLayer")
				.field("has_bias", &crate::dnn::LayerNormLayerTraitConst::has_bias(self))
				.field("axis", &crate::dnn::LayerNormLayerTraitConst::axis(self))
				.field("epsilon", &crate::dnn::LayerNormLayerTraitConst::epsilon(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::LogLayer>` instead, removal in Nov 2024"]
	pub type PtrOfLogLayer = core::Ptr<crate::dnn::LogLayer>;

	ptr_extern! { crate::dnn::LogLayer,
		cv_PtrLcv_dnn_LogLayerG_new_null_const, cv_PtrLcv_dnn_LogLayerG_delete, cv_PtrLcv_dnn_LogLayerG_getInnerPtr_const, cv_PtrLcv_dnn_LogLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::LogLayer, cv_PtrLcv_dnn_LogLayerG_new_const_LogLayer }
	impl core::Ptr<crate::dnn::LogLayer> {
		#[inline] pub fn as_raw_PtrOfLogLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLogLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::LogLayerTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_LogLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LogLayerTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_LogLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LogLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_LogLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LogLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_LogLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::LogLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::LogLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_LogLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::LogLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfLogLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::MVNLayer>` instead, removal in Nov 2024"]
	pub type PtrOfMVNLayer = core::Ptr<crate::dnn::MVNLayer>;

	ptr_extern! { crate::dnn::MVNLayer,
		cv_PtrLcv_dnn_MVNLayerG_new_null_const, cv_PtrLcv_dnn_MVNLayerG_delete, cv_PtrLcv_dnn_MVNLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MVNLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::MVNLayer, cv_PtrLcv_dnn_MVNLayerG_new_const_MVNLayer }
	impl core::Ptr<crate::dnn::MVNLayer> {
		#[inline] pub fn as_raw_PtrOfMVNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMVNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::MVNLayerTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_MVNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::MVNLayerTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_MVNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MVNLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MVNLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MVNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MVNLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MVNLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::MVNLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMVNLayer")
				.field("eps", &crate::dnn::MVNLayerTraitConst::eps(self))
				.field("norm_variance", &crate::dnn::MVNLayerTraitConst::norm_variance(self))
				.field("across_channels", &crate::dnn::MVNLayerTraitConst::across_channels(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::MatMulLayer>` instead, removal in Nov 2024"]
	pub type PtrOfMatMulLayer = core::Ptr<crate::dnn::MatMulLayer>;

	ptr_extern! { crate::dnn::MatMulLayer,
		cv_PtrLcv_dnn_MatMulLayerG_new_null_const, cv_PtrLcv_dnn_MatMulLayerG_delete, cv_PtrLcv_dnn_MatMulLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MatMulLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::MatMulLayer, cv_PtrLcv_dnn_MatMulLayerG_new_const_MatMulLayer }
	impl core::Ptr<crate::dnn::MatMulLayer> {
		#[inline] pub fn as_raw_PtrOfMatMulLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMatMulLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::MatMulLayerTraitConst for core::Ptr<crate::dnn::MatMulLayer> {
		#[inline] fn as_raw_MatMulLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::MatMulLayerTrait for core::Ptr<crate::dnn::MatMulLayer> {
		#[inline] fn as_raw_mut_MatMulLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MatMulLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MatMulLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MatMulLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MatMulLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MatMulLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MatMulLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MatMulLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MatMulLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::MatMulLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMatMulLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::MaxUnpoolLayer>` instead, removal in Nov 2024"]
	pub type PtrOfMaxUnpoolLayer = core::Ptr<crate::dnn::MaxUnpoolLayer>;

	ptr_extern! { crate::dnn::MaxUnpoolLayer,
		cv_PtrLcv_dnn_MaxUnpoolLayerG_new_null_const, cv_PtrLcv_dnn_MaxUnpoolLayerG_delete, cv_PtrLcv_dnn_MaxUnpoolLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MaxUnpoolLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::MaxUnpoolLayer, cv_PtrLcv_dnn_MaxUnpoolLayerG_new_const_MaxUnpoolLayer }
	impl core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] pub fn as_raw_PtrOfMaxUnpoolLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMaxUnpoolLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::MaxUnpoolLayerTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_MaxUnpoolLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::MaxUnpoolLayerTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_MaxUnpoolLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MaxUnpoolLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MaxUnpoolLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MaxUnpoolLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MaxUnpoolLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::MaxUnpoolLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMaxUnpoolLayer")
				.field("pool_kernel", &crate::dnn::MaxUnpoolLayerTraitConst::pool_kernel(self))
				.field("pool_pad", &crate::dnn::MaxUnpoolLayerTraitConst::pool_pad(self))
				.field("pool_stride", &crate::dnn::MaxUnpoolLayerTraitConst::pool_stride(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::MishLayer>` instead, removal in Nov 2024"]
	pub type PtrOfMishLayer = core::Ptr<crate::dnn::MishLayer>;

	ptr_extern! { crate::dnn::MishLayer,
		cv_PtrLcv_dnn_MishLayerG_new_null_const, cv_PtrLcv_dnn_MishLayerG_delete, cv_PtrLcv_dnn_MishLayerG_getInnerPtr_const, cv_PtrLcv_dnn_MishLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::MishLayer, cv_PtrLcv_dnn_MishLayerG_new_const_MishLayer }
	impl core::Ptr<crate::dnn::MishLayer> {
		#[inline] pub fn as_raw_PtrOfMishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfMishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::MishLayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::MishLayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MishLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_MishLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MishLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_MishLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::MishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::MishLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_MishLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::MishLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfMishLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::NaryEltwiseLayer>` instead, removal in Nov 2024"]
	pub type PtrOfNaryEltwiseLayer = core::Ptr<crate::dnn::NaryEltwiseLayer>;

	ptr_extern! { crate::dnn::NaryEltwiseLayer,
		cv_PtrLcv_dnn_NaryEltwiseLayerG_new_null_const, cv_PtrLcv_dnn_NaryEltwiseLayerG_delete, cv_PtrLcv_dnn_NaryEltwiseLayerG_getInnerPtr_const, cv_PtrLcv_dnn_NaryEltwiseLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::NaryEltwiseLayer, cv_PtrLcv_dnn_NaryEltwiseLayerG_new_const_NaryEltwiseLayer }
	impl core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] pub fn as_raw_PtrOfNaryEltwiseLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNaryEltwiseLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::NaryEltwiseLayerTraitConst for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_NaryEltwiseLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::NaryEltwiseLayerTrait for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_mut_NaryEltwiseLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::NaryEltwiseLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_NaryEltwiseLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::NaryEltwiseLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_NaryEltwiseLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::NaryEltwiseLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfNaryEltwiseLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::NormalizeBBoxLayer>` instead, removal in Nov 2024"]
	pub type PtrOfNormalizeBBoxLayer = core::Ptr<crate::dnn::NormalizeBBoxLayer>;

	ptr_extern! { crate::dnn::NormalizeBBoxLayer,
		cv_PtrLcv_dnn_NormalizeBBoxLayerG_new_null_const, cv_PtrLcv_dnn_NormalizeBBoxLayerG_delete, cv_PtrLcv_dnn_NormalizeBBoxLayerG_getInnerPtr_const, cv_PtrLcv_dnn_NormalizeBBoxLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::NormalizeBBoxLayer, cv_PtrLcv_dnn_NormalizeBBoxLayerG_new_const_NormalizeBBoxLayer }
	impl core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] pub fn as_raw_PtrOfNormalizeBBoxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNormalizeBBoxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::NormalizeBBoxLayerTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_NormalizeBBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::NormalizeBBoxLayerTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_NormalizeBBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::NormalizeBBoxLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_NormalizeBBoxLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::NormalizeBBoxLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_NormalizeBBoxLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::NormalizeBBoxLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfNormalizeBBoxLayer")
				.field("pnorm", &crate::dnn::NormalizeBBoxLayerTraitConst::pnorm(self))
				.field("epsilon", &crate::dnn::NormalizeBBoxLayerTraitConst::epsilon(self))
				.field("across_spatial", &crate::dnn::NormalizeBBoxLayerTraitConst::across_spatial(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::NotLayer>` instead, removal in Nov 2024"]
	pub type PtrOfNotLayer = core::Ptr<crate::dnn::NotLayer>;

	ptr_extern! { crate::dnn::NotLayer,
		cv_PtrLcv_dnn_NotLayerG_new_null_const, cv_PtrLcv_dnn_NotLayerG_delete, cv_PtrLcv_dnn_NotLayerG_getInnerPtr_const, cv_PtrLcv_dnn_NotLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::NotLayer, cv_PtrLcv_dnn_NotLayerG_new_const_NotLayer }
	impl core::Ptr<crate::dnn::NotLayer> {
		#[inline] pub fn as_raw_PtrOfNotLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfNotLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::NotLayerTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_NotLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::NotLayerTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_NotLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::NotLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_NotLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::NotLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_NotLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::NotLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::NotLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_NotLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::NotLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfNotLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::PaddingLayer>` instead, removal in Nov 2024"]
	pub type PtrOfPaddingLayer = core::Ptr<crate::dnn::PaddingLayer>;

	ptr_extern! { crate::dnn::PaddingLayer,
		cv_PtrLcv_dnn_PaddingLayerG_new_null_const, cv_PtrLcv_dnn_PaddingLayerG_delete, cv_PtrLcv_dnn_PaddingLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PaddingLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PaddingLayer, cv_PtrLcv_dnn_PaddingLayerG_new_const_PaddingLayer }
	impl core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] pub fn as_raw_PtrOfPaddingLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPaddingLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PaddingLayerTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_PaddingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PaddingLayerTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_PaddingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PaddingLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PaddingLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PaddingLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PaddingLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PaddingLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPaddingLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::PermuteLayer>` instead, removal in Nov 2024"]
	pub type PtrOfPermuteLayer = core::Ptr<crate::dnn::PermuteLayer>;

	ptr_extern! { crate::dnn::PermuteLayer,
		cv_PtrLcv_dnn_PermuteLayerG_new_null_const, cv_PtrLcv_dnn_PermuteLayerG_delete, cv_PtrLcv_dnn_PermuteLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PermuteLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PermuteLayer, cv_PtrLcv_dnn_PermuteLayerG_new_const_PermuteLayer }
	impl core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] pub fn as_raw_PtrOfPermuteLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPermuteLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PermuteLayerTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_PermuteLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PermuteLayerTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_PermuteLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PermuteLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PermuteLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PermuteLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PermuteLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PermuteLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPermuteLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::PoolingLayer>` instead, removal in Nov 2024"]
	pub type PtrOfPoolingLayer = core::Ptr<crate::dnn::PoolingLayer>;

	ptr_extern! { crate::dnn::PoolingLayer,
		cv_PtrLcv_dnn_PoolingLayerG_new_null_const, cv_PtrLcv_dnn_PoolingLayerG_delete, cv_PtrLcv_dnn_PoolingLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PoolingLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PoolingLayer, cv_PtrLcv_dnn_PoolingLayerG_new_const_PoolingLayer }
	impl core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] pub fn as_raw_PtrOfPoolingLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PoolingLayerTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PoolingLayerTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PoolingLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PoolingLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PoolingLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPoolingLayer")
				.field("typ", &crate::dnn::PoolingLayerTraitConst::typ(self))
				.field("kernel_size", &crate::dnn::PoolingLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::PoolingLayerTraitConst::strides(self))
				.field("pads_begin", &crate::dnn::PoolingLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::PoolingLayerTraitConst::pads_end(self))
				.field("global_pooling", &crate::dnn::PoolingLayerTraitConst::global_pooling(self))
				.field("is_global_pooling", &crate::dnn::PoolingLayerTraitConst::is_global_pooling(self))
				.field("compute_max_idx", &crate::dnn::PoolingLayerTraitConst::compute_max_idx(self))
				.field("pad_mode", &crate::dnn::PoolingLayerTraitConst::pad_mode(self))
				.field("ceil_mode", &crate::dnn::PoolingLayerTraitConst::ceil_mode(self))
				.field("ave_pool_padded_area", &crate::dnn::PoolingLayerTraitConst::ave_pool_padded_area(self))
				.field("pooled_size", &crate::dnn::PoolingLayerTraitConst::pooled_size(self))
				.field("spatial_scale", &crate::dnn::PoolingLayerTraitConst::spatial_scale(self))
				.field("ps_roi_out_channels", &crate::dnn::PoolingLayerTraitConst::ps_roi_out_channels(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::PoolingLayerInt8>` instead, removal in Nov 2024"]
	pub type PtrOfPoolingLayerInt8 = core::Ptr<crate::dnn::PoolingLayerInt8>;

	ptr_extern! { crate::dnn::PoolingLayerInt8,
		cv_PtrLcv_dnn_PoolingLayerInt8G_new_null_const, cv_PtrLcv_dnn_PoolingLayerInt8G_delete, cv_PtrLcv_dnn_PoolingLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_PoolingLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PoolingLayerInt8, cv_PtrLcv_dnn_PoolingLayerInt8G_new_const_PoolingLayerInt8 }
	impl core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] pub fn as_raw_PtrOfPoolingLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPoolingLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PoolingLayerInt8TraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_PoolingLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PoolingLayerInt8Trait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_PoolingLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PoolingLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PoolingLayerInt8G_to_PtrOfLayer }

	impl crate::dnn::PoolingLayerTraitConst for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_PoolingLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PoolingLayerTrait for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline] fn as_raw_mut_PoolingLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PoolingLayerInt8>, core::Ptr<crate::dnn::PoolingLayer>, cv_PtrLcv_dnn_PoolingLayerInt8G_to_PtrOfPoolingLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PoolingLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPoolingLayerInt8")
				.field("input_zp", &crate::dnn::PoolingLayerInt8TraitConst::input_zp(self))
				.field("output_zp", &crate::dnn::PoolingLayerInt8TraitConst::output_zp(self))
				.field("input_sc", &crate::dnn::PoolingLayerInt8TraitConst::input_sc(self))
				.field("output_sc", &crate::dnn::PoolingLayerInt8TraitConst::output_sc(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.field("typ", &crate::dnn::PoolingLayerTraitConst::typ(self))
				.field("kernel_size", &crate::dnn::PoolingLayerTraitConst::kernel_size(self))
				.field("strides", &crate::dnn::PoolingLayerTraitConst::strides(self))
				.field("pads_begin", &crate::dnn::PoolingLayerTraitConst::pads_begin(self))
				.field("pads_end", &crate::dnn::PoolingLayerTraitConst::pads_end(self))
				.field("global_pooling", &crate::dnn::PoolingLayerTraitConst::global_pooling(self))
				.field("is_global_pooling", &crate::dnn::PoolingLayerTraitConst::is_global_pooling(self))
				.field("compute_max_idx", &crate::dnn::PoolingLayerTraitConst::compute_max_idx(self))
				.field("pad_mode", &crate::dnn::PoolingLayerTraitConst::pad_mode(self))
				.field("ceil_mode", &crate::dnn::PoolingLayerTraitConst::ceil_mode(self))
				.field("ave_pool_padded_area", &crate::dnn::PoolingLayerTraitConst::ave_pool_padded_area(self))
				.field("pooled_size", &crate::dnn::PoolingLayerTraitConst::pooled_size(self))
				.field("spatial_scale", &crate::dnn::PoolingLayerTraitConst::spatial_scale(self))
				.field("ps_roi_out_channels", &crate::dnn::PoolingLayerTraitConst::ps_roi_out_channels(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::PowerLayer>` instead, removal in Nov 2024"]
	pub type PtrOfPowerLayer = core::Ptr<crate::dnn::PowerLayer>;

	ptr_extern! { crate::dnn::PowerLayer,
		cv_PtrLcv_dnn_PowerLayerG_new_null_const, cv_PtrLcv_dnn_PowerLayerG_delete, cv_PtrLcv_dnn_PowerLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PowerLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PowerLayer, cv_PtrLcv_dnn_PowerLayerG_new_const_PowerLayer }
	impl core::Ptr<crate::dnn::PowerLayer> {
		#[inline] pub fn as_raw_PtrOfPowerLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPowerLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PowerLayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PowerLayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PowerLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PowerLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PowerLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_PowerLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PowerLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PowerLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PowerLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PowerLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPowerLayer")
				.field("power", &crate::dnn::PowerLayerTraitConst::power(self))
				.field("scale", &crate::dnn::PowerLayerTraitConst::scale(self))
				.field("shift", &crate::dnn::PowerLayerTraitConst::shift(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::PriorBoxLayer>` instead, removal in Nov 2024"]
	pub type PtrOfPriorBoxLayer = core::Ptr<crate::dnn::PriorBoxLayer>;

	ptr_extern! { crate::dnn::PriorBoxLayer,
		cv_PtrLcv_dnn_PriorBoxLayerG_new_null_const, cv_PtrLcv_dnn_PriorBoxLayerG_delete, cv_PtrLcv_dnn_PriorBoxLayerG_getInnerPtr_const, cv_PtrLcv_dnn_PriorBoxLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::PriorBoxLayer, cv_PtrLcv_dnn_PriorBoxLayerG_new_const_PriorBoxLayer }
	impl core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] pub fn as_raw_PtrOfPriorBoxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfPriorBoxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::PriorBoxLayerTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_PriorBoxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::PriorBoxLayerTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_PriorBoxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PriorBoxLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_PriorBoxLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::PriorBoxLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_PriorBoxLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::PriorBoxLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfPriorBoxLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ProposalLayer>` instead, removal in Nov 2024"]
	pub type PtrOfProposalLayer = core::Ptr<crate::dnn::ProposalLayer>;

	ptr_extern! { crate::dnn::ProposalLayer,
		cv_PtrLcv_dnn_ProposalLayerG_new_null_const, cv_PtrLcv_dnn_ProposalLayerG_delete, cv_PtrLcv_dnn_ProposalLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ProposalLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ProposalLayer, cv_PtrLcv_dnn_ProposalLayerG_new_const_ProposalLayer }
	impl core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] pub fn as_raw_PtrOfProposalLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfProposalLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ProposalLayerTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_ProposalLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ProposalLayerTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_ProposalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ProposalLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ProposalLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ProposalLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ProposalLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ProposalLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfProposalLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::QuantizeDynamicLayer>` instead, removal in Nov 2024"]
	pub type PtrOfQuantizeDynamicLayer = core::Ptr<crate::dnn::QuantizeDynamicLayer>;

	ptr_extern! { crate::dnn::QuantizeDynamicLayer,
		cv_PtrLcv_dnn_QuantizeDynamicLayerG_new_null_const, cv_PtrLcv_dnn_QuantizeDynamicLayerG_delete, cv_PtrLcv_dnn_QuantizeDynamicLayerG_getInnerPtr_const, cv_PtrLcv_dnn_QuantizeDynamicLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::QuantizeDynamicLayer, cv_PtrLcv_dnn_QuantizeDynamicLayerG_new_const_QuantizeDynamicLayer }
	impl core::Ptr<crate::dnn::QuantizeDynamicLayer> {
		#[inline] pub fn as_raw_PtrOfQuantizeDynamicLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQuantizeDynamicLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::QuantizeDynamicLayerTraitConst for core::Ptr<crate::dnn::QuantizeDynamicLayer> {
		#[inline] fn as_raw_QuantizeDynamicLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::QuantizeDynamicLayerTrait for core::Ptr<crate::dnn::QuantizeDynamicLayer> {
		#[inline] fn as_raw_mut_QuantizeDynamicLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::QuantizeDynamicLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::QuantizeDynamicLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::QuantizeDynamicLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_QuantizeDynamicLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::QuantizeDynamicLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::QuantizeDynamicLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::QuantizeDynamicLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_QuantizeDynamicLayerG_to_PtrOfLayer }

	impl crate::dnn::QuantizeLayerTraitConst for core::Ptr<crate::dnn::QuantizeDynamicLayer> {
		#[inline] fn as_raw_QuantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::QuantizeLayerTrait for core::Ptr<crate::dnn::QuantizeDynamicLayer> {
		#[inline] fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::QuantizeDynamicLayer>, core::Ptr<crate::dnn::QuantizeLayer>, cv_PtrLcv_dnn_QuantizeDynamicLayerG_to_PtrOfQuantizeLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::QuantizeDynamicLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfQuantizeDynamicLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.field("scales", &crate::dnn::QuantizeLayerTraitConst::scales(self))
				.field("zeropoints", &crate::dnn::QuantizeLayerTraitConst::zeropoints(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::QuantizeLayer>` instead, removal in Nov 2024"]
	pub type PtrOfQuantizeLayer = core::Ptr<crate::dnn::QuantizeLayer>;

	ptr_extern! { crate::dnn::QuantizeLayer,
		cv_PtrLcv_dnn_QuantizeLayerG_new_null_const, cv_PtrLcv_dnn_QuantizeLayerG_delete, cv_PtrLcv_dnn_QuantizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_QuantizeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::QuantizeLayer, cv_PtrLcv_dnn_QuantizeLayerG_new_const_QuantizeLayer }
	impl core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] pub fn as_raw_PtrOfQuantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfQuantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::QuantizeLayerTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_QuantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::QuantizeLayerTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::QuantizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_QuantizeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::QuantizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_QuantizeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::QuantizeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfQuantizeLayer")
				.field("scales", &crate::dnn::QuantizeLayerTraitConst::scales(self))
				.field("zeropoints", &crate::dnn::QuantizeLayerTraitConst::zeropoints(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::RNNLayer>` instead, removal in Nov 2024"]
	pub type PtrOfRNNLayer = core::Ptr<crate::dnn::RNNLayer>;

	ptr_extern! { crate::dnn::RNNLayer,
		cv_PtrLcv_dnn_RNNLayerG_new_null_const, cv_PtrLcv_dnn_RNNLayerG_delete, cv_PtrLcv_dnn_RNNLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RNNLayerG_getInnerPtrMut
	}

	impl core::Ptr<crate::dnn::RNNLayer> {
		#[inline] pub fn as_raw_PtrOfRNNLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRNNLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::RNNLayerTraitConst for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_RNNLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::RNNLayerTrait for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_RNNLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RNNLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RNNLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RNNLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RNNLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RNNLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::RNNLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfRNNLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::RandomNormalLikeLayer>` instead, removal in Nov 2024"]
	pub type PtrOfRandomNormalLikeLayer = core::Ptr<crate::dnn::RandomNormalLikeLayer>;

	ptr_extern! { crate::dnn::RandomNormalLikeLayer,
		cv_PtrLcv_dnn_RandomNormalLikeLayerG_new_null_const, cv_PtrLcv_dnn_RandomNormalLikeLayerG_delete, cv_PtrLcv_dnn_RandomNormalLikeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RandomNormalLikeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::RandomNormalLikeLayer, cv_PtrLcv_dnn_RandomNormalLikeLayerG_new_const_RandomNormalLikeLayer }
	impl core::Ptr<crate::dnn::RandomNormalLikeLayer> {
		#[inline] pub fn as_raw_PtrOfRandomNormalLikeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRandomNormalLikeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::RandomNormalLikeLayerTraitConst for core::Ptr<crate::dnn::RandomNormalLikeLayer> {
		#[inline] fn as_raw_RandomNormalLikeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::RandomNormalLikeLayerTrait for core::Ptr<crate::dnn::RandomNormalLikeLayer> {
		#[inline] fn as_raw_mut_RandomNormalLikeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RandomNormalLikeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RandomNormalLikeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RandomNormalLikeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RandomNormalLikeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RandomNormalLikeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RandomNormalLikeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RandomNormalLikeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RandomNormalLikeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::RandomNormalLikeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfRandomNormalLikeLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ReLU6Layer>` instead, removal in Nov 2024"]
	pub type PtrOfReLU6Layer = core::Ptr<crate::dnn::ReLU6Layer>;

	ptr_extern! { crate::dnn::ReLU6Layer,
		cv_PtrLcv_dnn_ReLU6LayerG_new_null_const, cv_PtrLcv_dnn_ReLU6LayerG_delete, cv_PtrLcv_dnn_ReLU6LayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReLU6LayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ReLU6Layer, cv_PtrLcv_dnn_ReLU6LayerG_new_const_ReLU6Layer }
	impl core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] pub fn as_raw_PtrOfReLU6Layer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLU6Layer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ReLU6LayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ReLU6LayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLU6Layer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLU6Layer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLU6Layer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReLU6LayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ReLU6Layer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfReLU6Layer")
				.field("min_value", &crate::dnn::ReLU6LayerTraitConst::min_value(self))
				.field("max_value", &crate::dnn::ReLU6LayerTraitConst::max_value(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ReLULayer>` instead, removal in Nov 2024"]
	pub type PtrOfReLULayer = core::Ptr<crate::dnn::ReLULayer>;

	ptr_extern! { crate::dnn::ReLULayer,
		cv_PtrLcv_dnn_ReLULayerG_new_null_const, cv_PtrLcv_dnn_ReLULayerG_delete, cv_PtrLcv_dnn_ReLULayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReLULayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ReLULayer, cv_PtrLcv_dnn_ReLULayerG_new_const_ReLULayer }
	impl core::Ptr<crate::dnn::ReLULayer> {
		#[inline] pub fn as_raw_PtrOfReLULayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReLULayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ReLULayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ReLULayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLULayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReLULayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLULayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ReLULayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReLULayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReLULayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReLULayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ReLULayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfReLULayer")
				.field("negative_slope", &crate::dnn::ReLULayerTraitConst::negative_slope(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ReciprocalLayer>` instead, removal in Nov 2024"]
	pub type PtrOfReciprocalLayer = core::Ptr<crate::dnn::ReciprocalLayer>;

	ptr_extern! { crate::dnn::ReciprocalLayer,
		cv_PtrLcv_dnn_ReciprocalLayerG_new_null_const, cv_PtrLcv_dnn_ReciprocalLayerG_delete, cv_PtrLcv_dnn_ReciprocalLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReciprocalLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ReciprocalLayer, cv_PtrLcv_dnn_ReciprocalLayerG_new_const_ReciprocalLayer }
	impl core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] pub fn as_raw_PtrOfReciprocalLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReciprocalLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ReciprocalLayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_ReciprocalLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ReciprocalLayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_ReciprocalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReciprocalLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReciprocalLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReciprocalLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ReciprocalLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReciprocalLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReciprocalLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ReciprocalLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfReciprocalLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ReduceLayer>` instead, removal in Nov 2024"]
	pub type PtrOfReduceLayer = core::Ptr<crate::dnn::ReduceLayer>;

	ptr_extern! { crate::dnn::ReduceLayer,
		cv_PtrLcv_dnn_ReduceLayerG_new_null_const, cv_PtrLcv_dnn_ReduceLayerG_delete, cv_PtrLcv_dnn_ReduceLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReduceLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ReduceLayer, cv_PtrLcv_dnn_ReduceLayerG_new_const_ReduceLayer }
	impl core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] pub fn as_raw_PtrOfReduceLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReduceLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ReduceLayerTraitConst for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_ReduceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ReduceLayerTrait for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_mut_ReduceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReduceLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReduceLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReduceLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReduceLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ReduceLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfReduceLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::RegionLayer>` instead, removal in Nov 2024"]
	pub type PtrOfRegionLayer = core::Ptr<crate::dnn::RegionLayer>;

	ptr_extern! { crate::dnn::RegionLayer,
		cv_PtrLcv_dnn_RegionLayerG_new_null_const, cv_PtrLcv_dnn_RegionLayerG_delete, cv_PtrLcv_dnn_RegionLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RegionLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::RegionLayer, cv_PtrLcv_dnn_RegionLayerG_new_const_RegionLayer }
	impl core::Ptr<crate::dnn::RegionLayer> {
		#[inline] pub fn as_raw_PtrOfRegionLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRegionLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::RegionLayerTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_RegionLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::RegionLayerTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_RegionLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RegionLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RegionLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RegionLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RegionLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RegionLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::RegionLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfRegionLayer")
				.field("nms_threshold", &crate::dnn::RegionLayerTraitConst::nms_threshold(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ReorgLayer>` instead, removal in Nov 2024"]
	pub type PtrOfReorgLayer = core::Ptr<crate::dnn::ReorgLayer>;

	ptr_extern! { crate::dnn::ReorgLayer,
		cv_PtrLcv_dnn_ReorgLayerG_new_null_const, cv_PtrLcv_dnn_ReorgLayerG_delete, cv_PtrLcv_dnn_ReorgLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReorgLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ReorgLayer, cv_PtrLcv_dnn_ReorgLayerG_new_const_ReorgLayer }
	impl core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] pub fn as_raw_PtrOfReorgLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReorgLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ReorgLayerTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_ReorgLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ReorgLayerTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_ReorgLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReorgLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReorgLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReorgLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReorgLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ReorgLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfReorgLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::RequantizeLayer>` instead, removal in Nov 2024"]
	pub type PtrOfRequantizeLayer = core::Ptr<crate::dnn::RequantizeLayer>;

	ptr_extern! { crate::dnn::RequantizeLayer,
		cv_PtrLcv_dnn_RequantizeLayerG_new_null_const, cv_PtrLcv_dnn_RequantizeLayerG_delete, cv_PtrLcv_dnn_RequantizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RequantizeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::RequantizeLayer, cv_PtrLcv_dnn_RequantizeLayerG_new_const_RequantizeLayer }
	impl core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] pub fn as_raw_PtrOfRequantizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRequantizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::RequantizeLayerTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_RequantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::RequantizeLayerTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_RequantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RequantizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RequantizeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RequantizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RequantizeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::RequantizeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfRequantizeLayer")
				.field("scale", &crate::dnn::RequantizeLayerTraitConst::scale(self))
				.field("shift", &crate::dnn::RequantizeLayerTraitConst::shift(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ReshapeLayer>` instead, removal in Nov 2024"]
	pub type PtrOfReshapeLayer = core::Ptr<crate::dnn::ReshapeLayer>;

	ptr_extern! { crate::dnn::ReshapeLayer,
		cv_PtrLcv_dnn_ReshapeLayerG_new_null_const, cv_PtrLcv_dnn_ReshapeLayerG_delete, cv_PtrLcv_dnn_ReshapeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ReshapeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ReshapeLayer, cv_PtrLcv_dnn_ReshapeLayerG_new_const_ReshapeLayer }
	impl core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] pub fn as_raw_PtrOfReshapeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfReshapeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ReshapeLayerTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ReshapeLayerTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReshapeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ReshapeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ReshapeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ReshapeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ReshapeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfReshapeLayer")
				.field("new_shape_desc", &crate::dnn::ReshapeLayerTraitConst::new_shape_desc(self))
				.field("new_shape_range", &crate::dnn::ReshapeLayerTraitConst::new_shape_range(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ResizeLayer>` instead, removal in Nov 2024"]
	pub type PtrOfResizeLayer = core::Ptr<crate::dnn::ResizeLayer>;

	ptr_extern! { crate::dnn::ResizeLayer,
		cv_PtrLcv_dnn_ResizeLayerG_new_null_const, cv_PtrLcv_dnn_ResizeLayerG_delete, cv_PtrLcv_dnn_ResizeLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ResizeLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ResizeLayer, cv_PtrLcv_dnn_ResizeLayerG_new_const_ResizeLayer }
	impl core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] pub fn as_raw_PtrOfResizeLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfResizeLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ResizeLayerTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ResizeLayerTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ResizeLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ResizeLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ResizeLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ResizeLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ResizeLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfResizeLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::RoundLayer>` instead, removal in Nov 2024"]
	pub type PtrOfRoundLayer = core::Ptr<crate::dnn::RoundLayer>;

	ptr_extern! { crate::dnn::RoundLayer,
		cv_PtrLcv_dnn_RoundLayerG_new_null_const, cv_PtrLcv_dnn_RoundLayerG_delete, cv_PtrLcv_dnn_RoundLayerG_getInnerPtr_const, cv_PtrLcv_dnn_RoundLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::RoundLayer, cv_PtrLcv_dnn_RoundLayerG_new_const_RoundLayer }
	impl core::Ptr<crate::dnn::RoundLayer> {
		#[inline] pub fn as_raw_PtrOfRoundLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfRoundLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::RoundLayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_RoundLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::RoundLayerTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_RoundLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RoundLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_RoundLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RoundLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_RoundLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::RoundLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::RoundLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_RoundLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::RoundLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfRoundLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ScaleLayer>` instead, removal in Nov 2024"]
	pub type PtrOfScaleLayer = core::Ptr<crate::dnn::ScaleLayer>;

	ptr_extern! { crate::dnn::ScaleLayer,
		cv_PtrLcv_dnn_ScaleLayerG_new_null_const, cv_PtrLcv_dnn_ScaleLayerG_delete, cv_PtrLcv_dnn_ScaleLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ScaleLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ScaleLayer, cv_PtrLcv_dnn_ScaleLayerG_new_const_ScaleLayer }
	impl core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] pub fn as_raw_PtrOfScaleLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ScaleLayerTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ScaleLayerTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ScaleLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ScaleLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ScaleLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfScaleLayer")
				.field("has_bias", &crate::dnn::ScaleLayerTraitConst::has_bias(self))
				.field("axis", &crate::dnn::ScaleLayerTraitConst::axis(self))
				.field("mode", &crate::dnn::ScaleLayerTraitConst::mode(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ScaleLayerInt8>` instead, removal in Nov 2024"]
	pub type PtrOfScaleLayerInt8 = core::Ptr<crate::dnn::ScaleLayerInt8>;

	ptr_extern! { crate::dnn::ScaleLayerInt8,
		cv_PtrLcv_dnn_ScaleLayerInt8G_new_null_const, cv_PtrLcv_dnn_ScaleLayerInt8G_delete, cv_PtrLcv_dnn_ScaleLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ScaleLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ScaleLayerInt8, cv_PtrLcv_dnn_ScaleLayerInt8G_new_const_ScaleLayerInt8 }
	impl core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] pub fn as_raw_PtrOfScaleLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScaleLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ScaleLayerInt8TraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_ScaleLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ScaleLayerInt8Trait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_ScaleLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ScaleLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ScaleLayerInt8G_to_PtrOfLayer }

	impl crate::dnn::ScaleLayerTraitConst for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_ScaleLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ScaleLayerTrait for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline] fn as_raw_mut_ScaleLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScaleLayerInt8>, core::Ptr<crate::dnn::ScaleLayer>, cv_PtrLcv_dnn_ScaleLayerInt8G_to_PtrOfScaleLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ScaleLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfScaleLayerInt8")
				.field("output_sc", &crate::dnn::ScaleLayerInt8TraitConst::output_sc(self))
				.field("output_zp", &crate::dnn::ScaleLayerInt8TraitConst::output_zp(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.field("has_bias", &crate::dnn::ScaleLayerTraitConst::has_bias(self))
				.field("axis", &crate::dnn::ScaleLayerTraitConst::axis(self))
				.field("mode", &crate::dnn::ScaleLayerTraitConst::mode(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ScatterLayer>` instead, removal in Nov 2024"]
	pub type PtrOfScatterLayer = core::Ptr<crate::dnn::ScatterLayer>;

	ptr_extern! { crate::dnn::ScatterLayer,
		cv_PtrLcv_dnn_ScatterLayerG_new_null_const, cv_PtrLcv_dnn_ScatterLayerG_delete, cv_PtrLcv_dnn_ScatterLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ScatterLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ScatterLayer, cv_PtrLcv_dnn_ScatterLayerG_new_const_ScatterLayer }
	impl core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] pub fn as_raw_PtrOfScatterLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScatterLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ScatterLayerTraitConst for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_ScatterLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ScatterLayerTrait for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_mut_ScatterLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScatterLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ScatterLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScatterLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ScatterLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ScatterLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfScatterLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ScatterNDLayer>` instead, removal in Nov 2024"]
	pub type PtrOfScatterNDLayer = core::Ptr<crate::dnn::ScatterNDLayer>;

	ptr_extern! { crate::dnn::ScatterNDLayer,
		cv_PtrLcv_dnn_ScatterNDLayerG_new_null_const, cv_PtrLcv_dnn_ScatterNDLayerG_delete, cv_PtrLcv_dnn_ScatterNDLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ScatterNDLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ScatterNDLayer, cv_PtrLcv_dnn_ScatterNDLayerG_new_const_ScatterNDLayer }
	impl core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] pub fn as_raw_PtrOfScatterNDLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfScatterNDLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ScatterNDLayerTraitConst for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_ScatterNDLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ScatterNDLayerTrait for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_mut_ScatterNDLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScatterNDLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ScatterNDLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ScatterNDLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ScatterNDLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ScatterNDLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfScatterNDLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SeluLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSeluLayer = core::Ptr<crate::dnn::SeluLayer>;

	ptr_extern! { crate::dnn::SeluLayer,
		cv_PtrLcv_dnn_SeluLayerG_new_null_const, cv_PtrLcv_dnn_SeluLayerG_delete, cv_PtrLcv_dnn_SeluLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SeluLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SeluLayer, cv_PtrLcv_dnn_SeluLayerG_new_const_SeluLayer }
	impl core::Ptr<crate::dnn::SeluLayer> {
		#[inline] pub fn as_raw_PtrOfSeluLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSeluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SeluLayerTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_SeluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SeluLayerTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_SeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SeluLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SeluLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SeluLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SeluLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SeluLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SeluLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SeluLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SeluLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSeluLayer")
				.field("alpha", &crate::dnn::SeluLayerTraitConst::alpha(self))
				.field("gamma", &crate::dnn::SeluLayerTraitConst::gamma(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ShiftLayer>` instead, removal in Nov 2024"]
	pub type PtrOfShiftLayer = core::Ptr<crate::dnn::ShiftLayer>;

	ptr_extern! { crate::dnn::ShiftLayer,
		cv_PtrLcv_dnn_ShiftLayerG_new_null_const, cv_PtrLcv_dnn_ShiftLayerG_delete, cv_PtrLcv_dnn_ShiftLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ShiftLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ShiftLayer, cv_PtrLcv_dnn_ShiftLayerG_new_const_ShiftLayer }
	impl core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] pub fn as_raw_PtrOfShiftLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShiftLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ShiftLayerTraitConst for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_ShiftLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ShiftLayerTrait for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_mut_ShiftLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShiftLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShiftLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ShiftLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfShiftLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ShiftLayerInt8>` instead, removal in Nov 2024"]
	pub type PtrOfShiftLayerInt8 = core::Ptr<crate::dnn::ShiftLayerInt8>;

	ptr_extern! { crate::dnn::ShiftLayerInt8,
		cv_PtrLcv_dnn_ShiftLayerInt8G_new_null_const, cv_PtrLcv_dnn_ShiftLayerInt8G_delete, cv_PtrLcv_dnn_ShiftLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_ShiftLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ShiftLayerInt8, cv_PtrLcv_dnn_ShiftLayerInt8G_new_const_ShiftLayerInt8 }
	impl core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] pub fn as_raw_PtrOfShiftLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShiftLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ShiftLayerInt8TraitConst for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_ShiftLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ShiftLayerInt8Trait for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_mut_ShiftLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShiftLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShiftLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShiftLayerInt8G_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ShiftLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfShiftLayerInt8")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ShrinkLayer>` instead, removal in Nov 2024"]
	pub type PtrOfShrinkLayer = core::Ptr<crate::dnn::ShrinkLayer>;

	ptr_extern! { crate::dnn::ShrinkLayer,
		cv_PtrLcv_dnn_ShrinkLayerG_new_null_const, cv_PtrLcv_dnn_ShrinkLayerG_delete, cv_PtrLcv_dnn_ShrinkLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ShrinkLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ShrinkLayer, cv_PtrLcv_dnn_ShrinkLayerG_new_const_ShrinkLayer }
	impl core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] pub fn as_raw_PtrOfShrinkLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShrinkLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ShrinkLayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_ShrinkLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ShrinkLayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_ShrinkLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShrinkLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShrinkLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShrinkLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ShrinkLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShrinkLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShrinkLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ShrinkLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfShrinkLayer")
				.field("bias", &crate::dnn::ShrinkLayerTraitConst::bias(self))
				.field("lambd", &crate::dnn::ShrinkLayerTraitConst::lambd(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ShuffleChannelLayer>` instead, removal in Nov 2024"]
	pub type PtrOfShuffleChannelLayer = core::Ptr<crate::dnn::ShuffleChannelLayer>;

	ptr_extern! { crate::dnn::ShuffleChannelLayer,
		cv_PtrLcv_dnn_ShuffleChannelLayerG_new_null_const, cv_PtrLcv_dnn_ShuffleChannelLayerG_delete, cv_PtrLcv_dnn_ShuffleChannelLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ShuffleChannelLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ShuffleChannelLayer, cv_PtrLcv_dnn_ShuffleChannelLayerG_new_const_ShuffleChannelLayer }
	impl core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] pub fn as_raw_PtrOfShuffleChannelLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfShuffleChannelLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ShuffleChannelLayerTraitConst for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_ShuffleChannelLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ShuffleChannelLayerTrait for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_mut_ShuffleChannelLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShuffleChannelLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ShuffleChannelLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ShuffleChannelLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ShuffleChannelLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ShuffleChannelLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfShuffleChannelLayer")
				.field("group", &crate::dnn::ShuffleChannelLayerTraitConst::group(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SigmoidLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSigmoidLayer = core::Ptr<crate::dnn::SigmoidLayer>;

	ptr_extern! { crate::dnn::SigmoidLayer,
		cv_PtrLcv_dnn_SigmoidLayerG_new_null_const, cv_PtrLcv_dnn_SigmoidLayerG_delete, cv_PtrLcv_dnn_SigmoidLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SigmoidLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SigmoidLayer, cv_PtrLcv_dnn_SigmoidLayerG_new_const_SigmoidLayer }
	impl core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] pub fn as_raw_PtrOfSigmoidLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSigmoidLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SigmoidLayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_SigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SigmoidLayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SigmoidLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SigmoidLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SigmoidLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SigmoidLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SigmoidLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SigmoidLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SigmoidLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSigmoidLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SignLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSignLayer = core::Ptr<crate::dnn::SignLayer>;

	ptr_extern! { crate::dnn::SignLayer,
		cv_PtrLcv_dnn_SignLayerG_new_null_const, cv_PtrLcv_dnn_SignLayerG_delete, cv_PtrLcv_dnn_SignLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SignLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SignLayer, cv_PtrLcv_dnn_SignLayerG_new_const_SignLayer }
	impl core::Ptr<crate::dnn::SignLayer> {
		#[inline] pub fn as_raw_PtrOfSignLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSignLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SignLayerTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_SignLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SignLayerTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_SignLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SignLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SignLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SignLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SignLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SignLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SignLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SignLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SignLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSignLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SinLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSinLayer = core::Ptr<crate::dnn::SinLayer>;

	ptr_extern! { crate::dnn::SinLayer,
		cv_PtrLcv_dnn_SinLayerG_new_null_const, cv_PtrLcv_dnn_SinLayerG_delete, cv_PtrLcv_dnn_SinLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SinLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SinLayer, cv_PtrLcv_dnn_SinLayerG_new_const_SinLayer }
	impl core::Ptr<crate::dnn::SinLayer> {
		#[inline] pub fn as_raw_PtrOfSinLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SinLayerTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_SinLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SinLayerTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_SinLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SinLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SinLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SinLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SinLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SinLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SinLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SinLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SinLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSinLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SinhLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSinhLayer = core::Ptr<crate::dnn::SinhLayer>;

	ptr_extern! { crate::dnn::SinhLayer,
		cv_PtrLcv_dnn_SinhLayerG_new_null_const, cv_PtrLcv_dnn_SinhLayerG_delete, cv_PtrLcv_dnn_SinhLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SinhLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SinhLayer, cv_PtrLcv_dnn_SinhLayerG_new_const_SinhLayer }
	impl core::Ptr<crate::dnn::SinhLayer> {
		#[inline] pub fn as_raw_PtrOfSinhLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSinhLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SinhLayerTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_SinhLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SinhLayerTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_SinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SinhLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SinhLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SinhLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SinhLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SinhLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SinhLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SinhLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SinhLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSinhLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SliceLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSliceLayer = core::Ptr<crate::dnn::SliceLayer>;

	ptr_extern! { crate::dnn::SliceLayer,
		cv_PtrLcv_dnn_SliceLayerG_new_null_const, cv_PtrLcv_dnn_SliceLayerG_delete, cv_PtrLcv_dnn_SliceLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SliceLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SliceLayer, cv_PtrLcv_dnn_SliceLayerG_new_const_SliceLayer }
	impl core::Ptr<crate::dnn::SliceLayer> {
		#[inline] pub fn as_raw_PtrOfSliceLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSliceLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SliceLayerTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_SliceLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SliceLayerTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_SliceLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SliceLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SliceLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SliceLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SliceLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SliceLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SliceLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSliceLayer")
				.field("slice_ranges", &crate::dnn::SliceLayerTraitConst::slice_ranges(self))
				.field("slice_steps", &crate::dnn::SliceLayerTraitConst::slice_steps(self))
				.field("axis", &crate::dnn::SliceLayerTraitConst::axis(self))
				.field("num_split", &crate::dnn::SliceLayerTraitConst::num_split(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SoftmaxLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSoftmaxLayer = core::Ptr<crate::dnn::SoftmaxLayer>;

	ptr_extern! { crate::dnn::SoftmaxLayer,
		cv_PtrLcv_dnn_SoftmaxLayerG_new_null_const, cv_PtrLcv_dnn_SoftmaxLayerG_delete, cv_PtrLcv_dnn_SoftmaxLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SoftmaxLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SoftmaxLayer, cv_PtrLcv_dnn_SoftmaxLayerG_new_const_SoftmaxLayer }
	impl core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SoftmaxLayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SoftmaxLayerTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SoftmaxLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SoftmaxLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SoftmaxLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSoftmaxLayer")
				.field("log_soft_max", &crate::dnn::SoftmaxLayerTraitConst::log_soft_max(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SoftmaxLayerInt8>` instead, removal in Nov 2024"]
	pub type PtrOfSoftmaxLayerInt8 = core::Ptr<crate::dnn::SoftmaxLayerInt8>;

	ptr_extern! { crate::dnn::SoftmaxLayerInt8,
		cv_PtrLcv_dnn_SoftmaxLayerInt8G_new_null_const, cv_PtrLcv_dnn_SoftmaxLayerInt8G_delete, cv_PtrLcv_dnn_SoftmaxLayerInt8G_getInnerPtr_const, cv_PtrLcv_dnn_SoftmaxLayerInt8G_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SoftmaxLayerInt8, cv_PtrLcv_dnn_SoftmaxLayerInt8G_new_const_SoftmaxLayerInt8 }
	impl core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] pub fn as_raw_PtrOfSoftmaxLayerInt8(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftmaxLayerInt8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SoftmaxLayerInt8TraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_SoftmaxLayerInt8(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SoftmaxLayerInt8Trait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_SoftmaxLayerInt8(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayerInt8>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayerInt8>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfLayer }

	impl crate::dnn::SoftmaxLayerTraitConst for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_SoftmaxLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SoftmaxLayerTrait for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline] fn as_raw_mut_SoftmaxLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftmaxLayerInt8>, core::Ptr<crate::dnn::SoftmaxLayer>, cv_PtrLcv_dnn_SoftmaxLayerInt8G_to_PtrOfSoftmaxLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SoftmaxLayerInt8> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSoftmaxLayerInt8")
				.field("output_sc", &crate::dnn::SoftmaxLayerInt8TraitConst::output_sc(self))
				.field("output_zp", &crate::dnn::SoftmaxLayerInt8TraitConst::output_zp(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.field("log_soft_max", &crate::dnn::SoftmaxLayerTraitConst::log_soft_max(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SoftplusLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSoftplusLayer = core::Ptr<crate::dnn::SoftplusLayer>;

	ptr_extern! { crate::dnn::SoftplusLayer,
		cv_PtrLcv_dnn_SoftplusLayerG_new_null_const, cv_PtrLcv_dnn_SoftplusLayerG_delete, cv_PtrLcv_dnn_SoftplusLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SoftplusLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SoftplusLayer, cv_PtrLcv_dnn_SoftplusLayerG_new_const_SoftplusLayer }
	impl core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] pub fn as_raw_PtrOfSoftplusLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftplusLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SoftplusLayerTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_SoftplusLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SoftplusLayerTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_SoftplusLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftplusLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SoftplusLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftplusLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SoftplusLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftplusLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SoftplusLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SoftplusLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSoftplusLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SoftsignLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSoftsignLayer = core::Ptr<crate::dnn::SoftsignLayer>;

	ptr_extern! { crate::dnn::SoftsignLayer,
		cv_PtrLcv_dnn_SoftsignLayerG_new_null_const, cv_PtrLcv_dnn_SoftsignLayerG_delete, cv_PtrLcv_dnn_SoftsignLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SoftsignLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SoftsignLayer, cv_PtrLcv_dnn_SoftsignLayerG_new_const_SoftsignLayer }
	impl core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] pub fn as_raw_PtrOfSoftsignLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSoftsignLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SoftsignLayerTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_SoftsignLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SoftsignLayerTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_SoftsignLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftsignLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SoftsignLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftsignLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SoftsignLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SoftsignLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SoftsignLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SoftsignLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSoftsignLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SpaceToDepthLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSpaceToDepthLayer = core::Ptr<crate::dnn::SpaceToDepthLayer>;

	ptr_extern! { crate::dnn::SpaceToDepthLayer,
		cv_PtrLcv_dnn_SpaceToDepthLayerG_new_null_const, cv_PtrLcv_dnn_SpaceToDepthLayerG_delete, cv_PtrLcv_dnn_SpaceToDepthLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SpaceToDepthLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SpaceToDepthLayer, cv_PtrLcv_dnn_SpaceToDepthLayerG_new_const_SpaceToDepthLayer }
	impl core::Ptr<crate::dnn::SpaceToDepthLayer> {
		#[inline] pub fn as_raw_PtrOfSpaceToDepthLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSpaceToDepthLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SpaceToDepthLayerTraitConst for core::Ptr<crate::dnn::SpaceToDepthLayer> {
		#[inline] fn as_raw_SpaceToDepthLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SpaceToDepthLayerTrait for core::Ptr<crate::dnn::SpaceToDepthLayer> {
		#[inline] fn as_raw_mut_SpaceToDepthLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SpaceToDepthLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SpaceToDepthLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SpaceToDepthLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SpaceToDepthLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SpaceToDepthLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SpaceToDepthLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SpaceToDepthLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SpaceToDepthLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SpaceToDepthLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSpaceToDepthLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SplitLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSplitLayer = core::Ptr<crate::dnn::SplitLayer>;

	ptr_extern! { crate::dnn::SplitLayer,
		cv_PtrLcv_dnn_SplitLayerG_new_null_const, cv_PtrLcv_dnn_SplitLayerG_delete, cv_PtrLcv_dnn_SplitLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SplitLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SplitLayer, cv_PtrLcv_dnn_SplitLayerG_new_const_SplitLayer }
	impl core::Ptr<crate::dnn::SplitLayer> {
		#[inline] pub fn as_raw_PtrOfSplitLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSplitLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SplitLayerTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_SplitLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SplitLayerTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_SplitLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SplitLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SplitLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SplitLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SplitLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SplitLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SplitLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSplitLayer")
				.field("outputs_count", &crate::dnn::SplitLayerTraitConst::outputs_count(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SqrtLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSqrtLayer = core::Ptr<crate::dnn::SqrtLayer>;

	ptr_extern! { crate::dnn::SqrtLayer,
		cv_PtrLcv_dnn_SqrtLayerG_new_null_const, cv_PtrLcv_dnn_SqrtLayerG_delete, cv_PtrLcv_dnn_SqrtLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SqrtLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SqrtLayer, cv_PtrLcv_dnn_SqrtLayerG_new_const_SqrtLayer }
	impl core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] pub fn as_raw_PtrOfSqrtLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSqrtLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SqrtLayerTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_SqrtLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SqrtLayerTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_SqrtLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SqrtLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SqrtLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SqrtLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SqrtLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SqrtLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SqrtLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SqrtLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSqrtLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::SwishLayer>` instead, removal in Nov 2024"]
	pub type PtrOfSwishLayer = core::Ptr<crate::dnn::SwishLayer>;

	ptr_extern! { crate::dnn::SwishLayer,
		cv_PtrLcv_dnn_SwishLayerG_new_null_const, cv_PtrLcv_dnn_SwishLayerG_delete, cv_PtrLcv_dnn_SwishLayerG_getInnerPtr_const, cv_PtrLcv_dnn_SwishLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::SwishLayer, cv_PtrLcv_dnn_SwishLayerG_new_const_SwishLayer }
	impl core::Ptr<crate::dnn::SwishLayer> {
		#[inline] pub fn as_raw_PtrOfSwishLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfSwishLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::SwishLayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_SwishLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::SwishLayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_SwishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SwishLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_SwishLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SwishLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_SwishLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::SwishLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::SwishLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_SwishLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::SwishLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfSwishLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::TanHLayer>` instead, removal in Nov 2024"]
	pub type PtrOfTanHLayer = core::Ptr<crate::dnn::TanHLayer>;

	ptr_extern! { crate::dnn::TanHLayer,
		cv_PtrLcv_dnn_TanHLayerG_new_null_const, cv_PtrLcv_dnn_TanHLayerG_delete, cv_PtrLcv_dnn_TanHLayerG_getInnerPtr_const, cv_PtrLcv_dnn_TanHLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::TanHLayer, cv_PtrLcv_dnn_TanHLayerG_new_const_TanHLayer }
	impl core::Ptr<crate::dnn::TanHLayer> {
		#[inline] pub fn as_raw_PtrOfTanHLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTanHLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::TanHLayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_TanHLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::TanHLayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_TanHLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TanHLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_TanHLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TanHLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_TanHLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TanHLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TanHLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_TanHLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::TanHLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfTanHLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::TanLayer>` instead, removal in Nov 2024"]
	pub type PtrOfTanLayer = core::Ptr<crate::dnn::TanLayer>;

	ptr_extern! { crate::dnn::TanLayer,
		cv_PtrLcv_dnn_TanLayerG_new_null_const, cv_PtrLcv_dnn_TanLayerG_delete, cv_PtrLcv_dnn_TanLayerG_getInnerPtr_const, cv_PtrLcv_dnn_TanLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::TanLayer, cv_PtrLcv_dnn_TanLayerG_new_const_TanLayer }
	impl core::Ptr<crate::dnn::TanLayer> {
		#[inline] pub fn as_raw_PtrOfTanLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTanLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::TanLayerTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_TanLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::TanLayerTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_TanLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TanLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_TanLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TanLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_TanLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TanLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TanLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_TanLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::TanLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfTanLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::ThresholdedReluLayer>` instead, removal in Nov 2024"]
	pub type PtrOfThresholdedReluLayer = core::Ptr<crate::dnn::ThresholdedReluLayer>;

	ptr_extern! { crate::dnn::ThresholdedReluLayer,
		cv_PtrLcv_dnn_ThresholdedReluLayerG_new_null_const, cv_PtrLcv_dnn_ThresholdedReluLayerG_delete, cv_PtrLcv_dnn_ThresholdedReluLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ThresholdedReluLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::ThresholdedReluLayer, cv_PtrLcv_dnn_ThresholdedReluLayerG_new_const_ThresholdedReluLayer }
	impl core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] pub fn as_raw_PtrOfThresholdedReluLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfThresholdedReluLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::ThresholdedReluLayerTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_ThresholdedReluLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ThresholdedReluLayerTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_ThresholdedReluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ThresholdedReluLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ThresholdedReluLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::ActivationLayerTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::ActivationLayerTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ThresholdedReluLayer>, core::Ptr<crate::dnn::ActivationLayer>, cv_PtrLcv_dnn_ThresholdedReluLayerG_to_PtrOfActivationLayer }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::ThresholdedReluLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ThresholdedReluLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::ThresholdedReluLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfThresholdedReluLayer")
				.field("alpha", &crate::dnn::ThresholdedReluLayerTraitConst::alpha(self))
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::TileLayer>` instead, removal in Nov 2024"]
	pub type PtrOfTileLayer = core::Ptr<crate::dnn::TileLayer>;

	ptr_extern! { crate::dnn::TileLayer,
		cv_PtrLcv_dnn_TileLayerG_new_null_const, cv_PtrLcv_dnn_TileLayerG_delete, cv_PtrLcv_dnn_TileLayerG_getInnerPtr_const, cv_PtrLcv_dnn_TileLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::TileLayer, cv_PtrLcv_dnn_TileLayerG_new_const_TileLayer }
	impl core::Ptr<crate::dnn::TileLayer> {
		#[inline] pub fn as_raw_PtrOfTileLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTileLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::TileLayerTraitConst for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_TileLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::TileLayerTrait for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_mut_TileLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TileLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_TileLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TileLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TileLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_TileLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::TileLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfTileLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::dnn::TopKLayer>` instead, removal in Nov 2024"]
	pub type PtrOfTopKLayer = core::Ptr<crate::dnn::TopKLayer>;

	ptr_extern! { crate::dnn::TopKLayer,
		cv_PtrLcv_dnn_TopKLayerG_new_null_const, cv_PtrLcv_dnn_TopKLayerG_delete, cv_PtrLcv_dnn_TopKLayerG_getInnerPtr_const, cv_PtrLcv_dnn_TopKLayerG_getInnerPtrMut
	}

	ptr_extern_ctor! { crate::dnn::TopKLayer, cv_PtrLcv_dnn_TopKLayerG_new_const_TopKLayer }
	impl core::Ptr<crate::dnn::TopKLayer> {
		#[inline] pub fn as_raw_PtrOfTopKLayer(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfTopKLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::dnn::TopKLayerTraitConst for core::Ptr<crate::dnn::TopKLayer> {
		#[inline] fn as_raw_TopKLayer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::TopKLayerTrait for core::Ptr<crate::dnn::TopKLayer> {
		#[inline] fn as_raw_mut_TopKLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::TopKLayer> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::dnn::TopKLayer> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TopKLayer>, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_TopKLayerG_to_PtrOfAlgorithm }

	impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::TopKLayer> {
		#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::TopKLayer> {
		#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::dnn::TopKLayer>, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_TopKLayerG_to_PtrOfLayer }

	impl std::fmt::Debug for core::Ptr<crate::dnn::TopKLayer> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfTopKLayer")
				.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
				.field("name", &crate::dnn::LayerTraitConst::name(self))
				.field("typ", &crate::dnn::LayerTraitConst::typ(self))
				.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>` instead, removal in Nov 2024"]
	pub type TupleOfBackend_Target = core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>;

	impl core::Tuple<(crate::dnn::Backend, crate::dnn::Target)> {
		pub fn as_raw_TupleOfBackend_Target(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_TupleOfBackend_Target(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	tuple_extern! { (crate::dnn::Backend, crate::dnn::Target),
		std_pairLcv_dnn_Backend__cv_dnn_TargetG_new_const_Backend_Target, std_pairLcv_dnn_Backend__cv_dnn_TargetG_delete,
		0 = arg: crate::dnn::Backend, get_0 via std_pairLcv_dnn_Backend__cv_dnn_TargetG_get_0_const,
		1 = arg_1: crate::dnn::Target, get_1 via std_pairLcv_dnn_Backend__cv_dnn_TargetG_get_1_const
	}

	#[deprecated = "Use the the non-alias form `core::Vector<crate::dnn::MatShape>` instead, removal in Nov 2024"]
	pub type VectorOfMatShape = core::Vector<crate::dnn::MatShape>;

	impl core::Vector<crate::dnn::MatShape> {
		pub fn as_raw_VectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	#[deprecated = "Use the the non-alias form `core::Vector<core::Ptr<crate::dnn::BackendWrapper>>` instead, removal in Nov 2024"]
	pub type VectorOfPtrOfBackendWrapper = core::Vector<core::Ptr<crate::dnn::BackendWrapper>>;

	impl core::Vector<core::Ptr<crate::dnn::BackendWrapper>> {
		pub fn as_raw_VectorOfPtrOfBackendWrapper(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfBackendWrapper(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Ptr<crate::dnn::BackendWrapper>,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_new_const, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_delete,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_len_const, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_isEmpty_const,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_capacity_const, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_shrinkToFit,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_reserve_size_t, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_remove_size_t,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_clear,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_get_const_size_t, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_set_size_t_const_PtrLBackendWrapperG,
		std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_push_const_PtrLBackendWrapperG, std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_insert_size_t_const_PtrLBackendWrapperG,
	}

	vector_non_copy_or_bool! { core::Ptr<crate::dnn::BackendWrapper> }


	#[deprecated = "Use the the non-alias form `core::Vector<core::Ptr<crate::dnn::Layer>>` instead, removal in Nov 2024"]
	pub type VectorOfPtrOfLayer = core::Vector<core::Ptr<crate::dnn::Layer>>;

	impl core::Vector<core::Ptr<crate::dnn::Layer>> {
		pub fn as_raw_VectorOfPtrOfLayer(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfPtrOfLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Ptr<crate::dnn::Layer>,
		std_vectorLcv_PtrLcv_dnn_LayerGG_new_const, std_vectorLcv_PtrLcv_dnn_LayerGG_delete,
		std_vectorLcv_PtrLcv_dnn_LayerGG_len_const, std_vectorLcv_PtrLcv_dnn_LayerGG_isEmpty_const,
		std_vectorLcv_PtrLcv_dnn_LayerGG_capacity_const, std_vectorLcv_PtrLcv_dnn_LayerGG_shrinkToFit,
		std_vectorLcv_PtrLcv_dnn_LayerGG_reserve_size_t, std_vectorLcv_PtrLcv_dnn_LayerGG_remove_size_t,
		std_vectorLcv_PtrLcv_dnn_LayerGG_swap_size_t_size_t, std_vectorLcv_PtrLcv_dnn_LayerGG_clear,
		std_vectorLcv_PtrLcv_dnn_LayerGG_get_const_size_t, std_vectorLcv_PtrLcv_dnn_LayerGG_set_size_t_const_PtrLLayerG,
		std_vectorLcv_PtrLcv_dnn_LayerGG_push_const_PtrLLayerG, std_vectorLcv_PtrLcv_dnn_LayerGG_insert_size_t_const_PtrLLayerG,
	}

	vector_non_copy_or_bool! { core::Ptr<crate::dnn::Layer> }


	#[deprecated = "Use the the non-alias form `core::Vector<crate::dnn::Target>` instead, removal in Nov 2024"]
	pub type VectorOfTarget = core::Vector<crate::dnn::Target>;

	impl core::Vector<crate::dnn::Target> {
		pub fn as_raw_VectorOfTarget(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTarget(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { crate::dnn::Target,
		std_vectorLcv_dnn_TargetG_new_const, std_vectorLcv_dnn_TargetG_delete,
		std_vectorLcv_dnn_TargetG_len_const, std_vectorLcv_dnn_TargetG_isEmpty_const,
		std_vectorLcv_dnn_TargetG_capacity_const, std_vectorLcv_dnn_TargetG_shrinkToFit,
		std_vectorLcv_dnn_TargetG_reserve_size_t, std_vectorLcv_dnn_TargetG_remove_size_t,
		std_vectorLcv_dnn_TargetG_swap_size_t_size_t, std_vectorLcv_dnn_TargetG_clear,
		std_vectorLcv_dnn_TargetG_get_const_size_t, std_vectorLcv_dnn_TargetG_set_size_t_const_Target,
		std_vectorLcv_dnn_TargetG_push_const_Target, std_vectorLcv_dnn_TargetG_insert_size_t_const_Target,
	}

	vector_copy_non_bool! { crate::dnn::Target,
		std_vectorLcv_dnn_TargetG_data_const, std_vectorLcv_dnn_TargetG_dataMut, cv_fromSlice_const_const_TargetX_size_t,
		std_vectorLcv_dnn_TargetG_clone_const,
	}


	#[deprecated = "Use the the non-alias form `core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>>` instead, removal in Nov 2024"]
	pub type VectorOfTupleOfBackend_Target = core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>>;

	impl core::Vector<core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>> {
		pub fn as_raw_VectorOfTupleOfBackend_Target(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfTupleOfBackend_Target(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Tuple<(crate::dnn::Backend, crate::dnn::Target)>,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_new_const, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_delete,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_len_const, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_isEmpty_const,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_capacity_const, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_shrinkToFit,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_reserve_size_t, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_remove_size_t,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_swap_size_t_size_t, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_clear,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_get_const_size_t, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_set_size_t_const_pairLcv_dnn_Backend__cv_dnn_TargetG,
		std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_push_const_pairLcv_dnn_Backend__cv_dnn_TargetG, std_vectorLstd_pairLcv_dnn_Backend__cv_dnn_TargetGG_insert_size_t_const_pairLcv_dnn_Backend__cv_dnn_TargetG,
	}

	vector_non_copy_or_bool! { core::Tuple<(crate::dnn::Backend, crate::dnn::Target)> }


	#[deprecated = "Use the the non-alias form `core::Vector<core::Vector<crate::dnn::MatShape>>` instead, removal in Nov 2024"]
	pub type VectorOfVectorOfMatShape = core::Vector<core::Vector<crate::dnn::MatShape>>;

	impl core::Vector<core::Vector<crate::dnn::MatShape>> {
		pub fn as_raw_VectorOfVectorOfMatShape(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVectorOfMatShape(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { core::Vector<crate::dnn::MatShape>,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_new_const, std_vectorLstd_vectorLcv_dnn_MatShapeGG_delete,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_len_const, std_vectorLstd_vectorLcv_dnn_MatShapeGG_isEmpty_const,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_capacity_const, std_vectorLstd_vectorLcv_dnn_MatShapeGG_shrinkToFit,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_reserve_size_t, std_vectorLstd_vectorLcv_dnn_MatShapeGG_remove_size_t,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_dnn_MatShapeGG_clear,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_get_const_size_t, std_vectorLstd_vectorLcv_dnn_MatShapeGG_set_size_t_const_vectorLMatShapeG,
		std_vectorLstd_vectorLcv_dnn_MatShapeGG_push_const_vectorLMatShapeG, std_vectorLstd_vectorLcv_dnn_MatShapeGG_insert_size_t_const_vectorLMatShapeG,
	}

	vector_non_copy_or_bool! { core::Vector<crate::dnn::MatShape> }


}
#[cfg(ocvrs_has_module_dnn)]
pub use dnn_types::*;

#[cfg(ocvrs_has_module_imgproc)]
mod imgproc_types {
	use crate::{mod_prelude::*, core, types, sys};

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::imgproc::CLAHE>` instead, removal in Nov 2024"]
	pub type PtrOfCLAHE = core::Ptr<crate::imgproc::CLAHE>;

	ptr_extern! { crate::imgproc::CLAHE,
		cv_PtrLcv_CLAHEG_new_null_const, cv_PtrLcv_CLAHEG_delete, cv_PtrLcv_CLAHEG_getInnerPtr_const, cv_PtrLcv_CLAHEG_getInnerPtrMut
	}

	impl core::Ptr<crate::imgproc::CLAHE> {
		#[inline] pub fn as_raw_PtrOfCLAHE(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfCLAHE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::imgproc::CLAHETraitConst for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::CLAHETrait for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::CLAHE> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::CLAHE>, core::Ptr<core::Algorithm>, cv_PtrLcv_CLAHEG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<crate::imgproc::CLAHE> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfCLAHE")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::imgproc::GeneralizedHough>` instead, removal in Nov 2024"]
	pub type PtrOfGeneralizedHough = core::Ptr<crate::imgproc::GeneralizedHough>;

	ptr_extern! { crate::imgproc::GeneralizedHough,
		cv_PtrLcv_GeneralizedHoughG_new_null_const, cv_PtrLcv_GeneralizedHoughG_delete, cv_PtrLcv_GeneralizedHoughG_getInnerPtr_const, cv_PtrLcv_GeneralizedHoughG_getInnerPtrMut
	}

	impl core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline] pub fn as_raw_PtrOfGeneralizedHough(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHough(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHough>, core::Ptr<core::Algorithm>, cv_PtrLcv_GeneralizedHoughG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<crate::imgproc::GeneralizedHough> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGeneralizedHough")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::imgproc::GeneralizedHoughBallard>` instead, removal in Nov 2024"]
	pub type PtrOfGeneralizedHoughBallard = core::Ptr<crate::imgproc::GeneralizedHoughBallard>;

	ptr_extern! { crate::imgproc::GeneralizedHoughBallard,
		cv_PtrLcv_GeneralizedHoughBallardG_new_null_const, cv_PtrLcv_GeneralizedHoughBallardG_delete, cv_PtrLcv_GeneralizedHoughBallardG_getInnerPtr_const, cv_PtrLcv_GeneralizedHoughBallardG_getInnerPtrMut
	}

	impl core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughBallard(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::imgproc::GeneralizedHoughBallardTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::GeneralizedHoughBallardTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughBallard>, core::Ptr<core::Algorithm>, cv_PtrLcv_GeneralizedHoughBallardG_to_PtrOfAlgorithm }

	impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughBallard>, core::Ptr<crate::imgproc::GeneralizedHough>, cv_PtrLcv_GeneralizedHoughBallardG_to_PtrOfGeneralizedHough }

	impl std::fmt::Debug for core::Ptr<crate::imgproc::GeneralizedHoughBallard> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGeneralizedHoughBallard")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::imgproc::GeneralizedHoughGuil>` instead, removal in Nov 2024"]
	pub type PtrOfGeneralizedHoughGuil = core::Ptr<crate::imgproc::GeneralizedHoughGuil>;

	ptr_extern! { crate::imgproc::GeneralizedHoughGuil,
		cv_PtrLcv_GeneralizedHoughGuilG_new_null_const, cv_PtrLcv_GeneralizedHoughGuilG_delete, cv_PtrLcv_GeneralizedHoughGuilG_getInnerPtr_const, cv_PtrLcv_GeneralizedHoughGuilG_getInnerPtrMut
	}

	impl core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfGeneralizedHoughGuil(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::imgproc::GeneralizedHoughGuilTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::GeneralizedHoughGuilTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughGuil>, core::Ptr<core::Algorithm>, cv_PtrLcv_GeneralizedHoughGuilG_to_PtrOfAlgorithm }

	impl crate::imgproc::GeneralizedHoughTraitConst for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::GeneralizedHoughTrait for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::GeneralizedHoughGuil>, core::Ptr<crate::imgproc::GeneralizedHough>, cv_PtrLcv_GeneralizedHoughGuilG_to_PtrOfGeneralizedHough }

	impl std::fmt::Debug for core::Ptr<crate::imgproc::GeneralizedHoughGuil> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfGeneralizedHoughGuil")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::imgproc::LineSegmentDetector>` instead, removal in Nov 2024"]
	pub type PtrOfLineSegmentDetector = core::Ptr<crate::imgproc::LineSegmentDetector>;

	ptr_extern! { crate::imgproc::LineSegmentDetector,
		cv_PtrLcv_LineSegmentDetectorG_new_null_const, cv_PtrLcv_LineSegmentDetectorG_delete, cv_PtrLcv_LineSegmentDetectorG_getInnerPtr_const, cv_PtrLcv_LineSegmentDetectorG_getInnerPtrMut
	}

	impl core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfLineSegmentDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::imgproc::LineSegmentDetectorTraitConst for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_LineSegmentDetector(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::imgproc::LineSegmentDetectorTrait for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl core::AlgorithmTraitConst for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl core::AlgorithmTrait for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	ptr_cast_base! { core::Ptr<crate::imgproc::LineSegmentDetector>, core::Ptr<core::Algorithm>, cv_PtrLcv_LineSegmentDetectorG_to_PtrOfAlgorithm }

	impl std::fmt::Debug for core::Ptr<crate::imgproc::LineSegmentDetector> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfLineSegmentDetector")
				.finish()
		}
	}

}
#[cfg(ocvrs_has_module_imgproc)]
pub use imgproc_types::*;

#[cfg(ocvrs_has_module_videoio)]
mod videoio_types {
	use crate::{mod_prelude::*, core, types, sys};

	#[deprecated = "Use the the non-alias form `core::Ptr<crate::videoio::IStreamReader>` instead, removal in Nov 2024"]
	pub type PtrOfIStreamReader = core::Ptr<crate::videoio::IStreamReader>;

	ptr_extern! { crate::videoio::IStreamReader,
		cv_PtrLcv_IStreamReaderG_new_null_const, cv_PtrLcv_IStreamReaderG_delete, cv_PtrLcv_IStreamReaderG_getInnerPtr_const, cv_PtrLcv_IStreamReaderG_getInnerPtrMut
	}

	impl core::Ptr<crate::videoio::IStreamReader> {
		#[inline] pub fn as_raw_PtrOfIStreamReader(&self) -> extern_send!(Self) { self.as_raw() }
		#[inline] pub fn as_raw_mut_PtrOfIStreamReader(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	impl crate::videoio::IStreamReaderTraitConst for core::Ptr<crate::videoio::IStreamReader> {
		#[inline] fn as_raw_IStreamReader(&self) -> *const c_void { self.inner_as_raw() }
	}

	impl crate::videoio::IStreamReaderTrait for core::Ptr<crate::videoio::IStreamReader> {
		#[inline] fn as_raw_mut_IStreamReader(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
	}

	impl std::fmt::Debug for core::Ptr<crate::videoio::IStreamReader> {
		#[inline]
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			f.debug_struct("PtrOfIStreamReader")
				.finish()
		}
	}

	#[deprecated = "Use the the non-alias form `core::Vector<crate::videoio::VideoCaptureAPIs>` instead, removal in Nov 2024"]
	pub type VectorOfVideoCaptureAPIs = core::Vector<crate::videoio::VideoCaptureAPIs>;

	impl core::Vector<crate::videoio::VideoCaptureAPIs> {
		pub fn as_raw_VectorOfVideoCaptureAPIs(&self) -> extern_send!(Self) { self.as_raw() }
		pub fn as_raw_mut_VectorOfVideoCaptureAPIs(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
	}

	vector_extern! { crate::videoio::VideoCaptureAPIs,
		std_vectorLcv_VideoCaptureAPIsG_new_const, std_vectorLcv_VideoCaptureAPIsG_delete,
		std_vectorLcv_VideoCaptureAPIsG_len_const, std_vectorLcv_VideoCaptureAPIsG_isEmpty_const,
		std_vectorLcv_VideoCaptureAPIsG_capacity_const, std_vectorLcv_VideoCaptureAPIsG_shrinkToFit,
		std_vectorLcv_VideoCaptureAPIsG_reserve_size_t, std_vectorLcv_VideoCaptureAPIsG_remove_size_t,
		std_vectorLcv_VideoCaptureAPIsG_swap_size_t_size_t, std_vectorLcv_VideoCaptureAPIsG_clear,
		std_vectorLcv_VideoCaptureAPIsG_get_const_size_t, std_vectorLcv_VideoCaptureAPIsG_set_size_t_const_VideoCaptureAPIs,
		std_vectorLcv_VideoCaptureAPIsG_push_const_VideoCaptureAPIs, std_vectorLcv_VideoCaptureAPIsG_insert_size_t_const_VideoCaptureAPIs,
	}

	vector_copy_non_bool! { crate::videoio::VideoCaptureAPIs,
		std_vectorLcv_VideoCaptureAPIsG_data_const, std_vectorLcv_VideoCaptureAPIsG_dataMut, cv_fromSlice_const_const_VideoCaptureAPIsX_size_t,
		std_vectorLcv_VideoCaptureAPIsG_clone_const,
	}


}
#[cfg(ocvrs_has_module_videoio)]
pub use videoio_types::*;

pub use crate::manual::types::*;
